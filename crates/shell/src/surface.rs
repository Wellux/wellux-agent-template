//! The list the UI draws. A door appears because the core has it,
//! not because a template hard-coded a card.

use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Field {
    pub name: &'static str,
    pub label: &'static str,
    pub kind: &'static str,
    pub placeholder: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Action {
    pub id: &'static str,
    pub label: &'static str,
    pub method: &'static str,
    pub href: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Door {
    pub id: &'static str,
    pub title: &'static str,
    pub line: &'static str,
    pub connected: bool,
    pub missing: Vec<&'static str>,
    pub fields: Vec<Field>,
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Surface {
    pub project: String,
    pub repo: String,
    pub doors: Vec<Door>,
}

pub fn project_name(toml: &str) -> String {
    for line in toml.lines() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix("name") else {
            continue;
        };
        let rest = rest.trim().trim_start_matches('=').trim().trim_matches('"');
        if !rest.is_empty()
            && rest
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return rest.to_string();
        }
    }
    "wellux".to_string()
}

pub fn project_repo(toml: &str) -> String {
    for line in toml.lines() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix("repo") else {
            continue;
        };
        let rest = rest.trim().trim_start_matches('=').trim().trim_matches('"');
        if is_github_repo(rest) {
            return rest.to_string();
        }
    }
    String::new()
}

fn is_github_repo(value: &str) -> bool {
    let Some(rest) = value.strip_prefix("https://github.com/") else {
        return false;
    };
    let mut parts = rest.split('/');
    let owner = parts.next().unwrap_or("");
    let name = parts.next().unwrap_or("");
    parts.next().is_none()
        && !owner.is_empty()
        && !name.is_empty()
        && owner
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.')
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.')
        && !owner.starts_with('.')
        && !name.starts_with('.')
}

pub fn surface(project: &str, repo: &str, env: &BTreeMap<String, String>) -> Surface {
    Surface {
        project: project.to_string(),
        repo: if is_github_repo(repo) {
            repo.to_string()
        } else {
            String::new()
        },
        doors: vec![supabase(env), paypal(env), openai(env)],
    }
}

fn present(env: &BTreeMap<String, String>, key: &str) -> bool {
    env.get(key).is_some_and(|value| !value.trim().is_empty())
}

fn missing<'a>(env: &BTreeMap<String, String>, keys: &[&'a str]) -> Vec<&'a str> {
    keys.iter()
        .copied()
        .filter(|key| !present(env, key))
        .collect()
}

fn supabase(env: &BTreeMap<String, String>) -> Door {
    let need = ["SUPABASE_URL", "SUPABASE_PUBLISHABLE_KEY"];
    let gap = missing(env, &need);
    Door {
        id: "supabase",
        title: "Supabase",
        line: "Mail-Login und OAuth. Die Sitzung bleibt auf diesem Rechner.",
        connected: gap.is_empty(),
        missing: gap,
        fields: vec![
            Field {
                name: "email",
                label: "E-Mail",
                kind: "email",
                placeholder: "du@example.com",
            },
            Field {
                name: "password",
                label: "Passwort",
                kind: "password",
                placeholder: "",
            },
        ],
        actions: vec![
            Action {
                id: "password",
                label: "Anmelden",
                method: "POST",
                href: "/api/supabase/password",
            },
            Action {
                id: "github",
                label: "GitHub",
                method: "GET",
                href: "/api/supabase/oauth?provider=github",
            },
            Action {
                id: "google",
                label: "Google",
                method: "GET",
                href: "/api/supabase/oauth?provider=google",
            },
        ],
    }
}

fn paypal(env: &BTreeMap<String, String>) -> Door {
    let need = ["PAYPAL_CLIENT_ID", "PAYPAL_CLIENT_SECRET"];
    let gap = missing(env, &need);
    Door {
        id: "paypal",
        title: "PayPal",
        line: "Der Betrag wird hier gesetzt und an PayPal geschickt.",
        connected: gap.is_empty(),
        missing: gap,
        fields: vec![
            Field {
                name: "amount",
                label: "Betrag",
                kind: "text",
                placeholder: "10.50",
            },
            Field {
                name: "currency",
                label: "Währung",
                kind: "text",
                placeholder: "EUR",
            },
        ],
        actions: vec![Action {
            id: "order",
            label: "Zur Kasse",
            method: "POST",
            href: "/api/paypal/order",
        }],
    }
}

fn openai(env: &BTreeMap<String, String>) -> Door {
    let live = present(env, "OPENAI_API_KEY");
    Door {
        id: "openai",
        title: "OpenAI",
        line: "ChatGPT-Anmeldung startet ohne Schlüssel. Die Live-API braucht den Key oder eine Sitzung mit Plan-Scope.",
        connected: true,
        missing: if live { Vec::new() } else { vec!["OPENAI_API_KEY"] },
        fields: vec![],
        actions: vec![
            Action {
                id: "chatgpt",
                label: "Mit ChatGPT anmelden",
                method: "POST",
                href: "/api/chatgpt/start",
            },
            Action {
                id: "realtime",
                label: "Live-Secret holen",
                method: "POST",
                href: "/api/openai/realtime",
            },
        ],
    }
}

/// `"10.50"` → 1050. `"500"` for a zero-decimal currency stays 500.
pub fn major_to_minor(amount: &str, currency: &str) -> Result<u64, &'static str> {
    let amount = amount.trim();
    if amount.is_empty() || amount.starts_with('-') || amount.starts_with('+') {
        return Err("amount missing");
    }
    let zero_decimal = matches!(currency, "JPY" | "KRW");
    if zero_decimal {
        if amount.contains('.') {
            return Err("this currency has no subunits");
        }
        return amount.parse::<u64>().map_err(|_| "amount is not a number");
    }
    let (whole, frac) = match amount.split_once('.') {
        Some((whole, frac)) => (whole, frac),
        None => (amount, "00"),
    };
    if whole.is_empty() || !whole.bytes().all(|b| b.is_ascii_digit()) {
        return Err("amount is not a number");
    }
    let frac = match frac.len() {
        0 => "00".to_string(),
        1 => format!("{frac}0"),
        2 => frac.to_string(),
        _ => return Err("use at most two decimal places"),
    };
    if !frac.bytes().all(|b| b.is_ascii_digit()) {
        return Err("amount is not a number");
    }
    let major: u64 = whole.parse().map_err(|_| "amount is too large")?;
    let minor: u64 = frac.parse().map_err(|_| "amount is not a number")?;
    major
        .checked_mul(100)
        .and_then(|v| v.checked_add(minor))
        .ok_or("amount is too large")
}

pub fn load_dotenv(text: &str) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        if key.is_empty()
            || key
                .chars()
                .any(|c| !(c.is_ascii_alphanumeric() || c == '_'))
        {
            continue;
        }
        let value = value.trim().trim_matches('"');
        out.insert(key.to_string(), value.to_string());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect()
    }

    #[test]
    fn doors_follow_what_is_connected() {
        let bare = surface("wellux", "", &env(&[]));
        assert!(!bare.doors[0].connected);
        assert!(bare.doors[0].missing.contains(&"SUPABASE_URL"));
        assert!(!bare.doors[1].connected);

        let ready = surface(
            "wellux",
            "",
            &env(&[
                ("SUPABASE_URL", "https://example.supabase.co"),
                ("SUPABASE_PUBLISHABLE_KEY", "sb_publishable_x"),
                ("PAYPAL_CLIENT_ID", "id"),
                ("PAYPAL_CLIENT_SECRET", "secret"),
            ]),
        );
        assert!(ready.doors[0].connected);
        assert!(ready.doors[0].missing.is_empty());
        assert!(ready.doors[1].connected);
    }

    #[test]
    fn money_uses_minor_units() {
        assert_eq!(major_to_minor("10.50", "EUR").unwrap(), 1050);
        assert_eq!(major_to_minor("10", "EUR").unwrap(), 1000);
        assert_eq!(major_to_minor("500", "JPY").unwrap(), 500);
        assert!(major_to_minor("0.001", "EUR").is_err());
        assert!(major_to_minor("-1", "EUR").is_err());
    }

    #[test]
    fn dotenv_ignores_comments_and_blank_lines() {
        let map = load_dotenv("# note\n\nPAYPAL_CLIENT_ID=abc\nOPENAI_API_KEY=\"sk-test\"\n");
        assert_eq!(map.get("PAYPAL_CLIENT_ID").map(String::as_str), Some("abc"));
        assert_eq!(
            map.get("OPENAI_API_KEY").map(String::as_str),
            Some("sk-test")
        );
    }

    #[test]
    fn project_name_reads_the_manifest_and_rejects_noise() {
        assert_eq!(project_name("name = \"my-agent\"\n"), "my-agent");
        assert_eq!(project_name("name = \"no spaces\"\n"), "wellux");
        assert_eq!(project_name(""), "wellux");
        assert_eq!(
            project_repo("repo = \"https://github.com/acme/my-agent\"\n"),
            "https://github.com/acme/my-agent"
        );
        assert_eq!(project_repo("repo = \"https://evil.example/x\"\n"), "");
    }
}
