#[derive(Debug, Default, Clone)]
pub struct AgentSessionStore {
    ids: Vec<String>,
}

impl AgentSessionStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, id: impl Into<String>) {
        self.ids.push(id.into());
    }

    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    pub fn ids(&self) -> &[String] {
        &self.ids
    }
}
