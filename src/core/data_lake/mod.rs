use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct AuditEvent {
    pub id: u64,
    pub kind: String,
    pub source: String,
    pub message: String,
    pub timestamp_ms: u128,
}

#[derive(Clone, Default)]
pub struct DataLake {
    events: Arc<Mutex<Vec<AuditEvent>>>,
}

impl DataLake {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(
        &self,
        kind: impl Into<String>,
        source: impl Into<String>,
        message: impl Into<String>,
    ) -> AuditEvent {
        let mut events = self.events.lock().expect("data lake verrouillé");
        let event = AuditEvent {
            id: events.len() as u64 + 1,
            kind: kind.into(),
            source: source.into(),
            message: message.into(),
            timestamp_ms: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis(),
        };
        events.push(event.clone());
        event
    }

    pub fn list(&self, limit: usize) -> Vec<AuditEvent> {
        let events = self.events.lock().expect("data lake verrouillé");
        let start = events.len().saturating_sub(limit);
        events[start..].to_vec()
    }

    pub fn len(&self) -> usize {
        self.events.lock().expect("data lake verrouillé").len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conserve_les_evenements_dans_l_ordre() {
        let lake = DataLake::new();
        let first = lake.append("health", "core", "service démarré");
        lake.append("module", "cli", "module enregistré");
        assert_eq!(first.id, 1);
        assert_eq!(lake.len(), 2);
        assert_eq!(lake.list(1)[0].source, "cli");
    }
}
