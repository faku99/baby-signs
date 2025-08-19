use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use tuono_lib::{
    Request,
    cookie::{Cookie, CookieJar},
};

const COOKIE_NAME: &str = "bs_session"; // // TODO: Use env variable instead
const KEY: &[u8; 6] = b"secret"; // TODO: Use env variable instead

#[derive(Debug)]
pub struct BSCookie {
    claims: Claims,
}

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    iat: i64,
    exp: i64,
    session_id: String,
}

impl BSCookie {
    pub fn new(session_id: String) -> BSCookie {
        let iat = chrono::Local::now();
        let exp = iat + chrono::Duration::days(90);

        BSCookie {
            claims: Claims {
                iat: iat.timestamp_millis(),
                exp: exp.timestamp_millis(),
                session_id,
            },
        }
    }

    pub fn session_id(&self) -> String {
        self.claims.session_id.clone()
    }

    pub fn from_request(req: &Request) -> Option<BSCookie> {
        let jar = CookieJar::from_headers(&req.headers);
        let value = jar.get(COOKIE_NAME).map(|c| c.value().to_owned())?;

        let data = decode::<Claims>(
            &value,
            &DecodingKey::from_secret(KEY),
            &Validation::new(Algorithm::HS256),
        );
        if let Err(data) = &data {
            tracing::error!("{}", data);
            return None;
        }

        Some(BSCookie {
            claims: data.unwrap().claims,
        })
    }

    fn as_jwt(&self) -> Option<String> {
        let token = encode(
            &Header::default(),
            &self.claims,
            &EncodingKey::from_secret(KEY),
        );
        if let Err(token) = &token {
            tracing::error!("{}", token);
            return None;
        }

        Some(token.unwrap())
    }

    pub fn as_cookie(&self) -> Option<Cookie> {
        let jwt = self.as_jwt()?;

        Some(
            Cookie::build((COOKIE_NAME, jwt))
                .domain("localhost") // TODO: Use env variable instead
                .path("/")
                .secure(false)
                .http_only(true)
                .build(),
        )
    }
}
