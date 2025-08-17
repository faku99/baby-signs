use serde::Serialize;
use tuono_app::{cookie::BSCookie, models::session::{Session, SessionManager}};
use tuono_lib::{
    axum::http::StatusCode,
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

    let mut props = Props::new(SessionPageProps { session });

    let bs_cookie = BSCookie { session_id: session_id.to_string() };
    if let Some(cookie) = bs_cookie.as_cookie() {
        props.add_cookie(cookie);
    }

    Response::Props(props)
}
