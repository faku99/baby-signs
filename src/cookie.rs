use serde::{Deserialize, Serialize};
use tuono_lib::{cookie::CookieJar, Request};

const COOKIE_NAME: &str = "bs_session";

#[derive(Debug, Deserialize, Serialize)]
pub struct BSCookie {
    pub session_id: String,
}

impl BSCookie {
    pub fn from_request(req: &Request) -> Option<BSCookie> {
        let jar = CookieJar::from_headers(&req.headers);
        let cookie_str = jar.get(COOKIE_NAME).map(|c| c.value().to_owned())?;

        let cookie = serde_json::from_str(&cookie_str);
        if let Err(cookie) = &cookie {
            tracing::error!("{}", cookie);
            return None;
        }

        cookie.ok()
    }
}
