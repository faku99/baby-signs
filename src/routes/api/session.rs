use tuono_app::{cookie::BSCookie, models::{session::SessionManager, sign::Sign}};
use tuono_lib::{axum::{http::StatusCode, Json}, Request};

#[tuono_lib::api(GET)]
async fn session_random_sign(req: Request, session_manager: SessionManager) -> Result<Json<Sign>, StatusCode> {
    // If no cookie is present in the request, no need to go further.
    let Some(cookie) = BSCookie::from_request(&req) else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    // If session ID is not valid, early respond with NOT_FOUND.
    let Some(session) = session_manager.get_session(&cookie.session_id()) else {
        return Err(StatusCode::NOT_FOUND);
    };

    let Some(sign) = session.get_random_sign() else {
        return Err(StatusCode::NO_CONTENT);
    };

    Ok(Json(sign))
}

