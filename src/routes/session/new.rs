use tuono_app::{cookie::BSCookie, models::session::SessionManager};
use tuono_lib::{
    axum::http::{header, HeaderMap, StatusCode},
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
    if let Some(cookie) = BSCookie::from_request(&req) {
        return redirect_to_session(cookie.session_id);
    }

    let session = session_manager.create_session();
    redirect_to_session(session.id)
}
