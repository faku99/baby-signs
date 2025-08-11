use tuono_lib::{
    axum::http::{header, HeaderMap, StatusCode},
    cookie::CookieJar,
    Request, Response,
};

#[tuono_lib::handler]
async fn get_or_create_session(req: Request) -> Response {
    let mut session_id = "420".to_string(); // TODO: Change

    let jar = CookieJar::from_headers(&req.headers);
    if let Some(cookie_session_id) = jar.get("session-id").map(|c| c.value().to_owned()) {
        session_id = cookie_session_id.to_string();
    }

    let mut headers = HeaderMap::new();
    headers.insert(header::LOCATION, format!("/session/{}", session_id).parse().unwrap());

    Response::Custom((
        StatusCode::TEMPORARY_REDIRECT,
        headers,
        "Session ID not found".to_string(),
    ))
}
