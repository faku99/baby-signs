use serde::Serialize;
use tuono_lib::{cookie::Cookie, Props, Request, Response};

#[derive(Serialize)]
struct SessionPageProps {
    session_id: String,
}

#[tuono_lib::handler]
async fn get_session_page(req: Request) -> Response {
    let session_id = req.params.get("id").unwrap();

    let cookie = Cookie::build(("session-id", session_id))
        .domain("localhost")
        .path("/")
        .secure(false)
        .http_only(true)
        .build();

    let mut props = Props::new(SessionPageProps {
        session_id: session_id.to_string(),
    });
    props.add_cookie(cookie);

    Response::Props(props)
}
