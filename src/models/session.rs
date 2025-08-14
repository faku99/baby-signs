use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone, Debug, Default)]
pub struct Session {
    pub id: String,
}

impl Session {
    fn new() -> Session {
        Session {
            id: Uuid::new_v4().to_string(),
        }
    }
}

#[derive(Clone, Default)]
pub struct SessionManager {
    sessions: Arc<Mutex<HashMap<String, Session>>>,
}

impl SessionManager {
    pub fn new() -> SessionManager {
        SessionManager {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create_session(&self) -> Session {
        let session = Session::new();

        self.sessions
            .lock()
            .expect("Failed to acquire lock")
            .insert(session.id.clone(), session.clone());

        tracing::debug!("Created session with ID {}", session.id);

        session
    }

    pub fn get_session(&self, session_id: &str) -> Option<Session> {
        self.sessions
            .lock()
            .expect("Failed to acquire lock")
            .get(session_id)
            .cloned()
    }
}
