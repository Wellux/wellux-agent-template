//! Provider router. No network calls live here — only the table a shell uses
//! to point an OpenAI-compatible client at the right host and env key.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderKind {
    OpenAi,
    Anthropic,
    Gemini,
    Xai,
    OrcaRouter,
    Ollama,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderEndpoint {
    pub kind: ProviderKind,
    pub base_url: &'static str,
    pub env_key: &'static str,
}

impl ProviderKind {
    pub fn parse(name: &str) -> Option<Self> {
        match name.trim().to_ascii_lowercase().as_str() {
            "openai" => Some(Self::OpenAi),
            "anthropic" | "claude" => Some(Self::Anthropic),
            "gemini" | "google" => Some(Self::Gemini),
            "xai" | "grok" => Some(Self::Xai),
            "orcarouter" | "orca" => Some(Self::OrcaRouter),
            "ollama" => Some(Self::Ollama),
            _ => None,
        }
    }

    /// Recommended default for a new app: one key, provider cost, many models.
    pub const fn recommended() -> Self {
        Self::OrcaRouter
    }
}

pub fn create_provider(kind: ProviderKind) -> ProviderEndpoint {
    let (base_url, env_key) = match kind {
        ProviderKind::OpenAi => ("https://api.openai.com/v1", "OPENAI_API_KEY"),
        ProviderKind::Anthropic => ("https://api.anthropic.com/v1", "ANTHROPIC_API_KEY"),
        ProviderKind::Gemini => (
            "https://generativelanguage.googleapis.com/v1beta",
            "GEMINI_API_KEY",
        ),
        ProviderKind::Xai => ("https://api.x.ai/v1", "XAI_API_KEY"),
        ProviderKind::OrcaRouter => ("https://api.orcarouter.ai/v1", "ORCAROUTER_API_KEY"),
        ProviderKind::Ollama => ("http://127.0.0.1:11434/v1", "OLLAMA_HOST"),
    };
    ProviderEndpoint {
        kind,
        base_url,
        env_key,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orca_is_the_recommended_gateway() {
        let endpoint = create_provider(ProviderKind::recommended());
        assert_eq!(endpoint.kind, ProviderKind::OrcaRouter);
        assert_eq!(endpoint.base_url, "https://api.orcarouter.ai/v1");
        assert_eq!(endpoint.env_key, "ORCAROUTER_API_KEY");
    }

    #[test]
    fn parses_aliases() {
        assert_eq!(ProviderKind::parse(" grok "), Some(ProviderKind::Xai));
        assert_eq!(ProviderKind::parse("orca"), Some(ProviderKind::OrcaRouter));
        assert_eq!(ProviderKind::parse("nope"), None);
    }
}
