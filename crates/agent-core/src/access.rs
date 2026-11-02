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
}
