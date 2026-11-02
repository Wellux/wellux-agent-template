//! In-process stand-in for the libSQL session service.
//!
//! The type name stays `LibsqlSessionService` so shells written against the
//! template keep compiling when you swap this body for a real libSQL file
//! (`~/.wellux/sessions.db`). Nothing here opens a socket.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionRecord {
    pub id: String,
    pub summary: String,
}

#[derive(Debug, Default)]
pub struct LibsqlSessionService {
    rows: HashMap<String, SessionRecord>,
}

pub fn create_session_service() -> LibsqlSessionService {
    LibsqlSessionService::default()
}

impl LibsqlSessionService {
    pub fn upsert(&mut self, record: SessionRecord) {
        self.rows.insert(record.id.clone(), record);
    }

    pub fn get(&self, id: &str) -> Option<&SessionRecord> {
        self.rows.get(id)
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_without_a_database() {
        let mut svc = create_session_service();
        svc.upsert(SessionRecord {
            id: "s1".into(),
            summary: "first run".into(),
        });
        assert_eq!(
            svc.get("s1").map(|row| row.summary.as_str()),
            Some("first run")
        );
    }
}
