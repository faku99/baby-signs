use tuono_app::models::session::SessionManager;
use tuono_lib::{
    axum::http::{header, HeaderMap, StatusCode},
    cookie::CookieJar,
    Request, Response,
};

fn redirect_to_session(id: String) -> Response {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::LOCATION,
        format!("/session/{}", id).parse().unwrap(),
    );

    Response::Custom((StatusCode::TEMPORARY_REDIRECT, headers, String::new()))
}

#[tuono_lib::handler]
async fn get_or_create_session(req: Request, session_manager: SessionManager) -> Response {
    let jar = CookieJar::from_headers(&req.headers);
    if let Some(session_id) = jar.get("session-id").map(|c| c.value().to_owned()) {
        if let Some(session) = session_manager.get_session(&session_id) {
            return redirect_to_session(session.id);
        }
    }

    let session = session_manager.create_session();
    redirect_to_session(session.id)
}
