use serde::Serialize;
use tuono_lib::{
    // axum::http::{HeaderMap, StatusCode},
    // cookie::Cookie,
    Props, Request, Response,
};

#[derive(Serialize)]
struct SessionPageProps {
    session_id: String,
}

#[tuono_lib::handler]
async fn get_session_page(req: Request) -> Response {
    let session_id = req.params.get("id").unwrap();
    //if session_id == "new" {
    //    println!("Creating new session...");
    //    return Response::Custom((StatusCode::OK, HeaderMap::new(), "".to_string()));
    //}

    // let cookie = Cookie::build(("session-id", session_id))
    //     .domain("localhost")
    //     .path("/")
    //     .secure(false)
    //     .http_only(true)
    //     .build();

    let props = Props::new(SessionPageProps {
        session_id: session_id.to_string(),
    });
    // props.add_cookie(cookie);

    Response::Props(props)
}
