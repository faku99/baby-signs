use serde::Serialize;
use serde_json::{json, Value};
use tuono_app::{cookie::BSCookie, models::session::SessionManager};
use tuono_lib::{axum::Json, Request, Type};

#[derive(Serialize, Type)]
struct ApiSessionResponse {
    name: String,
}

#[tuono_lib::api(GET)]
async fn session_random_sign(req: Request, session_manager: SessionManager) -> Json<Value> {
    if let Some(cookie) = BSCookie::from_request(&req) {
        let _session = session_manager.get_session(&cookie.session_id());
    }

    Json(json!(ApiSessionResponse {
        name: "hello".to_string()
    }))
}
