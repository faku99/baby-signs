use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tuono_app::models::session::SessionManager;

#[derive(Clone)]
pub struct ApplicationState {
    pub session_manager: SessionManager,
}

pub fn main() -> ApplicationState {
    // Initialize logger with tracing.
    // We want to filter out logs from other crates so stdout doesn't get polluted.
    let filter = tracing_subscriber::filter::Targets::new()
        .with_target("tuono_app", Level::DEBUG)
        .with_default(Level::INFO);
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    let session_manager = SessionManager::new();

    ApplicationState { session_manager }
}
