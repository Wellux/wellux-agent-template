use crate::providers::ProviderKind;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentSession {
    pub id: String,
    pub provider: ProviderKind,
    pub approval_required: bool,
}

pub fn create_agent(provider: ProviderKind) -> AgentSession {
    AgentSession {
        id: Uuid::new_v4().to_string(),
        provider,
        approval_required: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_starts_closed_by_default() {
        let session = create_agent(ProviderKind::Ollama);
        assert_eq!(session.provider, ProviderKind::Ollama);
        assert!(session.approval_required);
        assert_eq!(session.id.len(), 36);
    }
}
