//! Where a credential is allowed to live.
//!
//! The core does not call Supabase, PayPal or OpenAI. It only records the
//! boundary so a shell cannot "just put the secret in the bundle" and still
//! claim it followed the template.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exposure {
    /// Safe in a browser bundle. Still not a secret.
    Browser,
    /// Process environment on the server. Never serialized to the client.
    Server,
    /// OS keychain or a `0600` file. Not a repo file, not a browser.
    Keychain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Boundary {
    pub name: &'static str,
    pub exposure: Exposure,
}

pub fn boundaries() -> &'static [Boundary] {
    &[
        Boundary {
            name: "SUPABASE_URL",
            exposure: Exposure::Browser,
        },
        Boundary {
            name: "SUPABASE_PUBLISHABLE_KEY",
            exposure: Exposure::Browser,
        },
        Boundary {
            name: "SUPABASE_SERVICE_ROLE_KEY",
            exposure: Exposure::Server,
        },
        Boundary {
            name: "PAYPAL_CLIENT_ID",
            exposure: Exposure::Browser,
        },
        Boundary {
            name: "PAYPAL_CLIENT_SECRET",
            exposure: Exposure::Server,
        },
        Boundary {
            name: "PAYPAL_WEBHOOK_ID",
            exposure: Exposure::Server,
        },
        Boundary {
            name: "OPENAI_API_KEY",
            exposure: Exposure::Server,
        },
        Boundary {
            name: "CHATGPT_REFRESH_TOKEN",
            exposure: Exposure::Keychain,
        },
        Boundary {
            name: "REALTIME_EPHEMERAL_SECRET",
            exposure: Exposure::Browser,
        },
    ]
}

pub fn may_enter_browser(name: &str) -> bool {
    boundaries()
        .iter()
        .find(|boundary| boundary.name == name)
        .is_some_and(|boundary| boundary.exposure == Exposure::Browser)
}

pub const CHATGPT_SCOPES: &str =
    "openid profile email offline_access resource.invoke chatgpt.tokens.use.direct";

const PLAN_SCOPE: &str = "chatgpt.tokens.use.direct";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessError(pub &'static str);

impl std::fmt::Display for AccessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

impl std::error::Error for AccessError {}

/// S256 challenge for Supabase OAuth and Sign in with ChatGPT.
/// `verifier` is 43–128 unreserved characters. The caller supplies the random bytes.
pub fn pkce_s256(verifier: &str) -> Result<String, AccessError> {
    if !(43..=128).contains(&verifier.len())
        || !verifier.bytes().all(
            |b| matches!(b, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~'),
        )
    {
        return Err(AccessError(
            "pkce verifier must be 43–128 unreserved characters",
        ));
    }
    use sha2::{Digest, Sha256};
    Ok(base64url(&Sha256::digest(verifier.as_bytes())))
}

pub fn plan_usage_granted(scopes: &str) -> bool {
    scopes.split_whitespace().any(|scope| scope == PLAN_SCOPE)
}

/// Exact redirect match. A `*` anywhere in the allow-list disables the check.
pub fn redirect_allowed(redirect: &str, allow: &[&str]) -> bool {
    if redirect.is_empty() || redirect.contains('*') {
        return false;
    }
    allow.iter().all(|item| !item.contains('*')) && allow.iter().any(|item| *item == redirect)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaypalOrder {
    pub currency_code: String,
    pub value: String,
}

impl PaypalOrder {
    /// Body for `POST /v2/checkout/orders`. The amount is already server-side.
    pub fn body(&self) -> String {
        format!(
            r#"{{"intent":"CAPTURE","purchase_units":[{{"amount":{{"currency_code":"{}","value":"{}"}}}}]}}"#,
            self.currency_code, self.value
        )
    }
}

/// `minor_units` is the smallest unit of `currency` (cents for EUR/USD, yen for JPY).
/// Zero is rejected. The browser never passes this number in.
pub fn paypal_order(currency: &str, minor_units: u64) -> Result<PaypalOrder, AccessError> {
    if currency.len() != 3 || !currency.bytes().all(|b| b.is_ascii_uppercase()) {
        return Err(AccessError("currency must be three uppercase letters"));
    }
    if minor_units == 0 {
        return Err(AccessError("amount must be greater than zero"));
    }
    let digits = match currency {
        "JPY" | "KRW" => 0,
        _ => 2,
    };
    let value = if digits == 0 {
        minor_units.to_string()
    } else {
        format!("{}.{:02}", minor_units / 100, minor_units % 100)
    };
    Ok(PaypalOrder {
        currency_code: currency.to_string(),
        value,
    })
}

#[derive(Debug, Clone)]
pub struct ChatGptLogin {
    pub client_id: String,
    pub redirect_uri: String,
    pub state: String,
    pub nonce: String,
    pub code_challenge: String,
    /// First launch of an open-source agent. Uses `dynamic_agent_client` once.
    pub first_registration: bool,
    pub agent_name: String,
    pub host_id: String,
}

pub fn chatgpt_authorize_url(login: &ChatGptLogin) -> Result<String, AccessError> {
    if login.state.trim().is_empty()
        || login.nonce.trim().is_empty()
        || login.code_challenge.trim().is_empty()
    {
        return Err(AccessError("state, nonce and code_challenge are required"));
    }
    if !loopback_or_https(&login.redirect_uri) {
        return Err(AccessError(
            "redirect must be https or a 127.0.0.1 loopback",
        ));
    }
    let client_id = if login.first_registration {
        if login.agent_name.trim().is_empty() || login.host_id.trim().is_empty() {
            return Err(AccessError(
                "first registration needs an agent name and host id",
            ));
        }
        "dynamic_agent_client"
    } else if login.client_id.starts_with("oaiapp_") {
        login.client_id.as_str()
    } else {
        return Err(AccessError(
            "reuse the issued oaiapp_ client id, not dynamic_agent_client",
        ));
    };
    let mut url = format!(
        "https://auth.openai.com/api/accounts/authorize?client_id={}&response_type=code&redirect_uri={}&scope={}&resource={}&state={}&nonce={}&code_challenge_method=S256&code_challenge={}",
        pct(client_id),
        pct(&login.redirect_uri),
        pct(CHATGPT_SCOPES),
        pct("https://api.openai.com/v1"),
        pct(&login.state),
        pct(&login.nonce),
        pct(&login.code_challenge),
    );
    if login.first_registration {
        url.push_str("&agent_name_hint=");
        url.push_str(&pct(&login.agent_name));
        url.push_str("&ext_agent_host_id=");
        url.push_str(&pct(&login.host_id));
    }
    Ok(url)
}

/// Accept a short-lived Realtime secret. Reject a durable API key.
pub fn browser_realtime_secret(secret: &str) -> Result<&str, AccessError> {
    let secret = secret.trim();
    if secret.is_empty() || secret.contains(['\n', '\r', ' ']) {
        return Err(AccessError("realtime secret is empty or malformed"));
    }
    let lower = secret.to_ascii_lowercase();
    if lower.starts_with("sk-") || lower.starts_with("xai-") {
        return Err(AccessError("durable API key must not reach the browser"));
    }
    Ok(secret)
}

fn loopback_or_https(uri: &str) -> bool {
    uri.starts_with("https://")
        || uri.starts_with("http://127.0.0.1/")
        || uri.starts_with("http://127.0.0.1:")
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

fn base64url(data: &[u8]) -> String {
    const TABLE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut out = String::new();
    let mut i = 0;
    while i + 3 <= data.len() {
        let n = ((data[i] as u32) << 16) | ((data[i + 1] as u32) << 8) | data[i + 2] as u32;
        out.push(TABLE[((n >> 18) & 63) as usize] as char);
        out.push(TABLE[((n >> 12) & 63) as usize] as char);
        out.push(TABLE[((n >> 6) & 63) as usize] as char);
        out.push(TABLE[(n & 63) as usize] as char);
        i += 3;
    }
    if i < data.len() {
        let rem = data.len() - i;
        let mut n = (data[i] as u32) << 16;
        if rem == 2 {
            n |= (data[i + 1] as u32) << 8;
        }
        out.push(TABLE[((n >> 18) & 63) as usize] as char);
        out.push(TABLE[((n >> 12) & 63) as usize] as char);
        if rem == 2 {
            out.push(TABLE[((n >> 6) & 63) as usize] as char);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn durable_secrets_never_ship_to_the_browser() {
        for name in [
            "SUPABASE_SERVICE_ROLE_KEY",
            "PAYPAL_CLIENT_SECRET",
            "PAYPAL_WEBHOOK_ID",
            "OPENAI_API_KEY",
            "CHATGPT_REFRESH_TOKEN",
        ] {
            assert!(!may_enter_browser(name), "{name}");
        }
    }

    #[test]
    fn the_browser_only_receives_public_ids_or_a_short_lived_realtime_secret() {
        assert!(may_enter_browser("SUPABASE_PUBLISHABLE_KEY"));
        assert!(may_enter_browser("PAYPAL_CLIENT_ID"));
        assert!(may_enter_browser("REALTIME_EPHEMERAL_SECRET"));
    }

    #[test]
    fn pkce_matches_the_rfc_vector() {
        let challenge = pkce_s256("dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk").unwrap();
        assert_eq!(challenge, "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM");
        assert!(pkce_s256("short").is_err());
    }

    #[test]
    fn plan_scope_is_a_whole_token() {
        assert!(plan_usage_granted(
            "openid profile chatgpt.tokens.use.direct"
        ));
        assert!(!plan_usage_granted("openid email"));
        assert!(!plan_usage_granted("notchatgpt.tokens.use.direct"));
    }

    #[test]
    fn redirects_are_exact_and_wildcards_fail_closed() {
        let allow = ["https://app.example/auth/callback"];
        assert!(redirect_allowed(
            "https://app.example/auth/callback",
            &allow
        ));
        assert!(!redirect_allowed(
            "https://app.example/auth/callback/extra",
            &allow
        ));
        assert!(!redirect_allowed(
            "https://app.example/auth/callback",
            &["https://*"]
        ));
    }

    #[test]
    fn paypal_amount_is_server_units_and_never_zero() {
        let order = paypal_order("EUR", 1050).unwrap();
        assert_eq!(order.value, "10.50");
        assert!(order.body().contains("\"value\":\"10.50\""));
        assert!(paypal_order("EUR", 0).is_err());
        assert_eq!(paypal_order("JPY", 500).unwrap().value, "500");
    }

    #[test]
    fn chatgpt_login_refuses_a_stored_dynamic_client_and_a_raw_key() {
        let login = ChatGptLogin {
            client_id: "oaiapp_example".into(),
            redirect_uri: "http://127.0.0.1:1455/auth/callback".into(),
            state: "state".into(),
            nonce: "nonce".into(),
            code_challenge: "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM".into(),
            first_registration: false,
            agent_name: String::new(),
            host_id: String::new(),
        };
        let url = chatgpt_authorize_url(&login).unwrap();
        assert!(url.starts_with("https://auth.openai.com/api/accounts/authorize?"));
        assert!(url.contains("code_challenge_method=S256"));
        assert!(chatgpt_authorize_url(&ChatGptLogin {
            client_id: "dynamic_agent_client".into(),
            ..login.clone()
        })
        .is_err());
        assert!(browser_realtime_secret("sk-live-secret").is_err());
        assert_eq!(
            browser_realtime_secret("ek_short_lived").unwrap(),
            "ek_short_lived"
        );
    }
}
