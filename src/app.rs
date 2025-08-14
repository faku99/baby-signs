use tuono_app::models::session::SessionManager;

#[derive(Clone)]
pub struct ApplicationState {
    pub session_manager: SessionManager,
}

pub fn main() -> ApplicationState {
    let session_manager = SessionManager::new();

    ApplicationState { session_manager }
}
