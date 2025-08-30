use chrono::Local;
use rand::seq::IndexedRandom;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tuono_lib::Type;
use uuid::Uuid;

use crate::models::sign::Sign;

#[derive(Clone, Debug, Default, Serialize, Type)]
pub enum SessionStatus {
    #[default]
    ACTIVE,
    COMPLETED,
}

#[derive(Clone, Debug, Default, Serialize, Type)]
pub struct Session {
    pub id: String,
    pub completed_signs: Vec<String>,
    pub started_at: String,
    pub status: SessionStatus,
}

impl Session {
    fn new() -> Session {
        Session {
            id: Uuid::new_v4().to_string(),
            completed_signs: Vec::new(),
            started_at: Local::now().to_string(),
            status: SessionStatus::default(),
        }
    }

    pub fn get_random_sign(&self) -> Option<Sign> {
        let unlearnt_signs: Vec<Sign> = Sign::signs()
            .into_iter()
            .filter(|s| !self.completed_signs.contains(&s.id.to_string()))
            .collect();
        if unlearnt_signs.is_empty() {
            return None;
        }

        let mut rng = rand::rng();
        unlearnt_signs.choose(&mut rng).cloned()
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

        tracing::debug!("Created session: {session:?}");

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
