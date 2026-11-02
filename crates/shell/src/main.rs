//! Local shell. Cards come from [`surface`], not from a fixed layout.
//! Bind is loopback only. Tokens stay in memory and never go back out.

mod surface;

use agent_core::{
    browser_realtime_secret, chatgpt_authorize_url, paypal_order, pkce_s256, plan_usage_granted,
    ChatGptLogin,
};
use axum::{
    extract::{Query, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    Json, Router,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{collections::BTreeMap, fs, net::SocketAddr, path::PathBuf, sync::Arc, time::Duration};
use tokio::sync::Mutex;
use uuid::Uuid;

struct PendingLogin {
    verifier: String,
    nonce: String,
    client_id: String,
    redirect_uri: String,
}

struct Vault {
    chatgpt_access: Option<String>,
    chatgpt_plan: bool,
    supabase_email: Option<String>,
    pending: BTreeMap<String, PendingLogin>,
}

struct App {
    env: BTreeMap<String, String>,
    ui: PathBuf,
    origin: String,
    vault: Mutex<Vault>,
    http: reqwest::Client,
}

#[tokio::main]
async fn main() {
    let mut env = surface::load_dotenv(&fs::read_to_string(".env").unwrap_or_default());
    for (key, value) in std::env::vars() {
        if !value.is_empty() {
            env.insert(key, value);
        }
    }
    let port: u16 = env
        .get("WELLUX_SHELL_PORT")
        .and_then(|v| v.parse().ok())
        .unwrap_or(8787);
    let origin = format!("http://127.0.0.1:{port}");
    let ui = ui_root();
    let app = Arc::new(App {
        env,
        ui,
        origin: origin.clone(),
        vault: Mutex::new(Vault {
            chatgpt_access: None,
            chatgpt_plan: false,
            supabase_email: None,
            pending: BTreeMap::new(),
        }),
        http: reqwest::Client::builder()
            .timeout(Duration::from_secs(20))
            .build()
            .expect("http client"),
    });
    let router = Router::new()
        .route("/", get(index))
        .route("/theme.css", get(css))
        .route("/app.js", get(js))
        .route("/api/surface", get(api_surface))
        .route("/api/session", get(api_session))
        .route("/api/paypal/order", post(paypal_order_route))
        .route("/paypal/return", get(paypal_return))
        .route("/paypal/cancel", get(paypal_cancel))
        .route("/api/chatgpt/start", post(chatgpt_start))
        .route("/auth/callback", get(chatgpt_callback))
        .route("/api/supabase/password", post(supabase_password))
        .route("/api/supabase/oauth", get(supabase_oauth))
        .route("/auth/supabase", get(index))
        .route("/api/supabase/adopt", post(supabase_adopt))
        .route("/api/openai/realtime", post(realtime))
        .with_state(app.clone());
    let addr: SocketAddr = format!("127.0.0.1:{port}").parse().expect("addr");
    println!("Wellux shell  {origin}");
    println!("UI folder     {}", app.ui.display());
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");
    axum::serve(listener, router).await.expect("serve");
}

fn ui_root() -> PathBuf {
    for candidate in ["ui", "../ui", "../../ui"] {
        let path = PathBuf::from(candidate);
        if path.join("index.html").is_file() {
            return path;
        }
    }
    PathBuf::from("ui")
}

fn file(app: &App, name: &str, mime: &str) -> Response {
    match fs::read(app.ui.join(name)) {
        Ok(bytes) => ([(header::CONTENT_TYPE, mime)], bytes).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("missing {name} — run make shell from the repo root"),
        )
            .into_response(),
    }
}

async fn index(State(app): State<Arc<App>>) -> impl IntoResponse {
    let mut response = file(&app, "index.html", "text/html; charset=utf-8");
    let token = Uuid::new_v4().simple().to_string();
    response.headers_mut().insert(
        header::SET_COOKIE,
        format!("wellux_csrf={token}; Path=/; SameSite=Lax")
            .parse()
            .expect("cookie"),
    );
    response
}

async fn css(State(app): State<Arc<App>>) -> impl IntoResponse {
    file(&app, "theme.css", "text/css; charset=utf-8")
}

async fn js(State(app): State<Arc<App>>) -> impl IntoResponse {
    file(&app, "app.js", "text/javascript; charset=utf-8")
}

async fn api_surface(State(app): State<Arc<App>>) -> Json<surface::Surface> {
    Json(surface::surface(&app.env))
}

async fn api_session(State(app): State<Arc<App>>) -> Json<Value> {
    let vault = app.vault.lock().await;
    Json(json!({
        "supabase": { "signed_in": vault.supabase_email.is_some(), "email": vault.supabase_email },
        "chatgpt": { "signed_in": vault.chatgpt_access.is_some(), "plan": vault.chatgpt_plan },
    }))
}

fn csrf_ok(headers: &HeaderMap) -> bool {
    let cookie = headers
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|raw| {
            raw.split(';').find_map(|part| {
                let part = part.trim();
                part.strip_prefix("wellux_csrf=").map(str::to_string)
            })
        });
    let header = headers
        .get("x-wellux-csrf")
        .and_then(|v| v.to_str().ok())
        .map(str::to_string);
    match (cookie, header) {
        (Some(cookie), Some(header)) if !cookie.is_empty() && cookie == header => true,
        _ => false,
    }
}

fn deny_csrf() -> Response {
    (
        StatusCode::FORBIDDEN,
        Json(json!({"error": "reload the page and try again"})),
    )
        .into_response()
}

#[derive(Deserialize)]
struct PaypalForm {
    amount: String,
    currency: String,
}

async fn paypal_order_route(
    State(app): State<Arc<App>>,
    headers: HeaderMap,
    Json(form): Json<PaypalForm>,
) -> Response {
    if !csrf_ok(&headers) {
        return deny_csrf();
    }
    let currency = form.currency.trim().to_ascii_uppercase();
    let minor = match surface::major_to_minor(&form.amount, &currency) {
        Ok(value) => value,
        Err(error) => {
            return (StatusCode::BAD_REQUEST, Json(json!({"error": error}))).into_response()
        }
    };
    let order = match paypal_order(&currency, minor) {
        Ok(order) => order,
        Err(error) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": error.to_string()})),
            )
                .into_response()
        }
    };
    let Some(secret) = app
        .env
        .get("PAYPAL_CLIENT_SECRET")
        .filter(|v| !v.is_empty())
    else {
        return (
            StatusCode::PRECONDITION_FAILED,
            Json(json!({"error": "PayPal is not connected", "body": order.body()})),
        )
            .into_response();
    };
    let Some(client_id) = app.env.get("PAYPAL_CLIENT_ID").filter(|v| !v.is_empty()) else {
        return (
            StatusCode::PRECONDITION_FAILED,
            Json(json!({"error": "PayPal is not connected"})),
        )
            .into_response();
    };
    let host = paypal_host(&app.env);
    let return_url = format!("{}/paypal/return", app.origin);
    let cancel_url = format!("{}/paypal/cancel", app.origin);
    let body = format!(
        r#"{{"intent":"CAPTURE","purchase_units":[{{"amount":{{"currency_code":"{}","value":"{}"}}}}],"application_context":{{"return_url":"{}","cancel_url":"{}","user_action":"PAY_NOW"}}}}"#,
        order.currency_code, order.value, return_url, cancel_url
    );
    let token = match paypal_token(&app.http, host, client_id, secret).await {
        Ok(token) => token,
        Err(error) => {
            return (StatusCode::BAD_GATEWAY, Json(json!({"error": error}))).into_response()
        }
    };
    let response = app
        .http
        .post(format!("{host}/v2/checkout/orders"))
        .bearer_auth(&token)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await;
    match response {
        Ok(response) => {
            let status = response.status();
            let payload: Value = response.json().await.unwrap_or(json!({}));
            if !status.is_success() {
                return (
                    StatusCode::BAD_GATEWAY,
                    Json(json!({"error": "PayPal refused the order"})),
                )
                    .into_response();
            }
            let approve = payload["links"]
                .as_array()
                .and_then(|links| links.iter().find(|link| link["rel"] == "approve"))
                .and_then(|link| link["href"].as_str())
                .unwrap_or("");
            if approve.is_empty() {
                return (
                    StatusCode::BAD_GATEWAY,
                    Json(json!({"error": "PayPal returned no approval link"})),
                )
                    .into_response();
            }
            (StatusCode::OK, Json(json!({"approve": approve}))).into_response()
        }
        Err(_) => (
            StatusCode::BAD_GATEWAY,
            Json(json!({"error": "PayPal unreachable"})),
        )
            .into_response(),
    }
}

async fn paypal_return(
    State(app): State<Arc<App>>,
    Query(query): Query<BTreeMap<String, String>>,
) -> Response {
    let Some(order_id) = query.get("token").cloned() else {
        return Redirect::to("/?notice=paypal-failed").into_response();
    };
    if !order_id.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Redirect::to("/?notice=paypal-failed").into_response();
    }
    let Some(secret) = app.env.get("PAYPAL_CLIENT_SECRET").cloned() else {
        return Redirect::to("/?notice=paypal-failed").into_response();
    };
    let Some(client_id) = app.env.get("PAYPAL_CLIENT_ID").cloned() else {
        return Redirect::to("/?notice=paypal-failed").into_response();
    };
    let host = paypal_host(&app.env);
    let Ok(token) = paypal_token(&app.http, host, &client_id, &secret).await else {
        return Redirect::to("/?notice=paypal-failed").into_response();
    };
    let sent = app
        .http
        .post(format!("{host}/v2/checkout/orders/{order_id}/capture"))
        .bearer_auth(token)
        .header("Content-Type", "application/json")
        .body("{}")
        .send()
        .await;
    match sent {
        Ok(response) if response.status().is_success() => {
            Redirect::to("/?notice=paypal-captured").into_response()
        }
        _ => Redirect::to("/?notice=paypal-failed").into_response(),
    }
}

async fn paypal_cancel() -> Redirect {
    Redirect::to("/?notice=paypal-cancelled")
}

async fn chatgpt_start(State(app): State<Arc<App>>, headers: HeaderMap) -> Response {
    if !csrf_ok(&headers) {
        return deny_csrf();
    }
    let verifier = format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple());
    let challenge = match pkce_s256(&verifier) {
        Ok(value) => value,
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": error.to_string()})),
            )
                .into_response()
        }
    };
    let state = Uuid::new_v4().simple().to_string();
    let nonce = Uuid::new_v4().simple().to_string();
    let redirect_uri = format!("{}/auth/callback", app.origin);
    let issued = app
        .env
        .get("CHATGPT_CLIENT_ID")
        .map(|s| s.trim().to_string())
        .filter(|s| s.starts_with("oaiapp_"));
    let login = ChatGptLogin {
        client_id: issued.clone().unwrap_or_default(),
        redirect_uri: redirect_uri.clone(),
        state: state.clone(),
        nonce: nonce.clone(),
        code_challenge: challenge,
        first_registration: issued.is_none(),
        agent_name: app
            .env
            .get("WELLUX_AGENT_NAME")
            .cloned()
            .unwrap_or_else(|| "Wellux".into()),
        host_id: app
            .env
            .get("WELLUX_HOST_ID")
            .cloned()
            .unwrap_or_else(|| "wellux-shell".into()),
    };
    let url = match chatgpt_authorize_url(&login) {
        Ok(url) => url,
        Err(error) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": error.to_string()})),
            )
                .into_response()
        }
    };
    app.vault.lock().await.pending.insert(
        state,
        PendingLogin {
            verifier,
            nonce,
            client_id: issued.unwrap_or_else(|| "dynamic_agent_client".into()),
            redirect_uri,
        },
    );
    (StatusCode::OK, Json(json!({"url": url}))).into_response()
}

async fn chatgpt_callback(
    State(app): State<Arc<App>>,
    Query(query): Query<BTreeMap<String, String>>,
) -> Response {
    let Some(state) = query.get("state").cloned() else {
        return Redirect::to("/?notice=chatgpt-failed").into_response();
    };
    let Some(code) = query.get("code").cloned() else {
        return Redirect::to("/?notice=chatgpt-failed").into_response();
    };
    let Some(pending) = app.vault.lock().await.pending.remove(&state) else {
        return Redirect::to("/?notice=chatgpt-failed").into_response();
    };
    let client_id = query
        .get("client_id")
        .cloned()
        .filter(|id| id.starts_with("oaiapp_"))
        .unwrap_or(pending.client_id);
    if client_id == "dynamic_agent_client" {
        return Redirect::to("/?notice=chatgpt-failed").into_response();
    }
    let body = format!(
        "grant_type=authorization_code&client_id={}&code={}&code_verifier={}&redirect_uri={}&resource={}",
        pct(&client_id),
        pct(&code),
        pct(&pending.verifier),
        pct(&pending.redirect_uri),
        pct("https://api.openai.com/v1"),
    );
    let sent = app
        .http
        .post("https://auth.openai.com/api/accounts/oauth/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await;
    let Ok(response) = sent else {
        return Redirect::to("/?notice=chatgpt-failed").into_response();
    };
    if !response.status().is_success() {
        return Redirect::to("/?notice=chatgpt-failed").into_response();
    }
    let payload: Value = response.json().await.unwrap_or(json!({}));
    let Some(access) = payload["access_token"].as_str().map(str::to_string) else {
        return Redirect::to("/?notice=chatgpt-failed").into_response();
    };
    let scope = payload["scope"].as_str().unwrap_or("");
    let plan = plan_usage_granted(scope);
    let _ = pending.nonce;
    {
        let mut vault = app.vault.lock().await;
        vault.chatgpt_access = Some(access);
        vault.chatgpt_plan = plan;
    }
    Redirect::to(if plan {
        "/?notice=chatgpt-plan"
    } else {
        "/?notice=chatgpt-signed-in"
    })
    .into_response()
}

#[derive(Deserialize)]
struct PasswordForm {
    email: String,
    password: String,
}

async fn supabase_password(
    State(app): State<Arc<App>>,
    headers: HeaderMap,
    Json(form): Json<PasswordForm>,
) -> Response {
    if !csrf_ok(&headers) {
        return deny_csrf();
    }
    let Some(base) = app
        .env
        .get("SUPABASE_URL")
        .filter(|v| v.starts_with("https://"))
    else {
        return (
            StatusCode::PRECONDITION_FAILED,
            Json(json!({"error": "Supabase is not connected"})),
        )
            .into_response();
    };
    let Some(key) = app
        .env
        .get("SUPABASE_PUBLISHABLE_KEY")
        .filter(|v| !v.is_empty())
    else {
        return (
            StatusCode::PRECONDITION_FAILED,
            Json(json!({"error": "Supabase is not connected"})),
        )
            .into_response();
    };
    let sent = app
        .http
        .post(format!(
            "{}/auth/v1/token?grant_type=password",
            base.trim_end_matches('/')
        ))
        .header("apikey", key)
        .header("Content-Type", "application/json")
        .json(&json!({"email": form.email, "password": form.password}))
        .send()
        .await;
    let Ok(response) = sent else {
        return (
            StatusCode::BAD_GATEWAY,
            Json(json!({"error": "Supabase unreachable"})),
        )
            .into_response();
    };
    if !response.status().is_success() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "login refused"})),
        )
            .into_response();
    }
    let payload: Value = response.json().await.unwrap_or(json!({}));
    let email = payload["user"]["email"]
        .as_str()
        .unwrap_or(form.email.as_str())
        .to_string();
    app.vault.lock().await.supabase_email = Some(email.clone());
    Json(json!({"signed_in": true, "email": email})).into_response()
}

#[derive(Deserialize)]
struct OauthQuery {
    provider: String,
}

async fn supabase_oauth(State(app): State<Arc<App>>, Query(query): Query<OauthQuery>) -> Response {
    if !matches!(query.provider.as_str(), "github" | "google" | "apple") {
        return (StatusCode::BAD_REQUEST, "unknown provider").into_response();
    }
    let Some(base) = app
        .env
        .get("SUPABASE_URL")
        .filter(|v| v.starts_with("https://"))
    else {
        return (StatusCode::PRECONDITION_FAILED, "Supabase is not connected").into_response();
    };
    let redirect = format!("{}/auth/supabase", app.origin);
    let url = format!(
        "{}/auth/v1/authorize?provider={}&redirect_to={}",
        base.trim_end_matches('/'),
        query.provider,
        pct(&redirect)
    );
    Redirect::to(&url).into_response()
}

#[derive(Deserialize)]
struct AdoptForm {
    access_token: String,
    email: Option<String>,
}

async fn supabase_adopt(
    State(app): State<Arc<App>>,
    headers: HeaderMap,
    Json(form): Json<AdoptForm>,
) -> Response {
    if !csrf_ok(&headers) {
        return deny_csrf();
    }
    if form.access_token.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "missing token"})),
        )
            .into_response();
    }
    app.vault.lock().await.supabase_email = Some(form.email.unwrap_or_else(|| "oauth".into()));
    Json(json!({"signed_in": true})).into_response()
}

async fn realtime(State(app): State<Arc<App>>, headers: HeaderMap) -> Response {
    if !csrf_ok(&headers) {
        return deny_csrf();
    }
    let vault = app.vault.lock().await;
    let bearer = if vault.chatgpt_plan {
        vault.chatgpt_access.clone()
    } else {
        None
    };
    let bearer = bearer.or_else(|| app.env.get("OPENAI_API_KEY").cloned());
    drop(vault);
    let Some(bearer) = bearer.filter(|v| !v.is_empty()) else {
        return (
            StatusCode::PRECONDITION_FAILED,
            Json(json!({"error": "no ChatGPT plan session and no API key"})),
        )
            .into_response();
    };
    let sent = app
        .http
        .post("https://api.openai.com/v1/realtime/client_secrets")
        .bearer_auth(bearer)
        .json(&json!({
            "expires_after": { "anchor": "created_at", "seconds": 600 },
            "session": { "type": "realtime", "model": "gpt-realtime" }
        }))
        .send()
        .await;
    let Ok(response) = sent else {
        return (
            StatusCode::BAD_GATEWAY,
            Json(json!({"error": "OpenAI unreachable"})),
        )
            .into_response();
    };
    if !response.status().is_success() {
        return (
            StatusCode::BAD_GATEWAY,
            Json(json!({"error": "OpenAI refused the live secret"})),
        )
            .into_response();
    }
    let payload: Value = response.json().await.unwrap_or(json!({}));
    let Some(value) = payload["value"].as_str() else {
        return (
            StatusCode::BAD_GATEWAY,
            Json(json!({"error": "OpenAI returned no secret"})),
        )
            .into_response();
    };
    match browser_realtime_secret(value) {
        Ok(secret) => Json(json!({"secret": secret, "expires_in": 600})).into_response(),
        Err(error) => (
            StatusCode::BAD_GATEWAY,
            Json(json!({"error": error.to_string()})),
        )
            .into_response(),
    }
}

fn paypal_host(env: &BTreeMap<String, String>) -> &'static str {
    if env
        .get("PAYPAL_ENV")
        .map(|v| v.eq_ignore_ascii_case("live"))
        .unwrap_or(false)
    {
        "https://api-m.paypal.com"
    } else {
        "https://api-m.sandbox.paypal.com"
    }
}

async fn paypal_token(
    http: &reqwest::Client,
    host: &str,
    id: &str,
    secret: &str,
) -> Result<String, String> {
    let basic = STANDARD.encode(format!("{id}:{secret}"));
    let response = http
        .post(format!("{host}/v1/oauth2/token"))
        .header("Authorization", format!("Basic {basic}"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body("grant_type=client_credentials")
        .send()
        .await
        .map_err(|_| "PayPal unreachable".to_string())?;
    if !response.status().is_success() {
        return Err("PayPal refused the client credentials".into());
    }
    let payload: Value = response
        .json()
        .await
        .map_err(|_| "PayPal returned no token".to_string())?;
    payload["access_token"]
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| "PayPal returned no token".into())
}

fn pct(input: &str) -> String {
    let mut out = String::new();
    for b in input.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}
