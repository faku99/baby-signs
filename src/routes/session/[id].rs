use serde::Serialize;
use tuono_app::models::session::{Session, SessionManager};
use tuono_lib::{
    axum::http::StatusCode,
    cookie::Cookie,
    Props, Request, Response, Type,
};

#[derive(Serialize, Type)]
struct SessionPageProps {
    session: Session,
}

#[tuono_lib::handler]
async fn get_session_page(req: Request, session_manager: SessionManager) -> Response {
    let session_id = req.params.get("id").unwrap();

    let Some(session) = session_manager.get_session(session_id) else {
        return Response::Props(Props::new_with_status("{}", StatusCode::NOT_FOUND));
    };

    let cookie = Cookie::build(("session-id", session.id.clone()))
        .domain("localhost")
        .path("/")
        .secure(false)
        .http_only(true)
        .build();

    let mut props = Props::new(SessionPageProps { session });
    props.add_cookie(cookie);

    Response::Props(props)
}
