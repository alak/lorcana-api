use actix_utils::future::{ready, Ready};
use actix_web::{http, Error, FromRequest, HttpRequest, Result};

use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;

#[derive(Debug, Default, Clone)]
pub struct BearerToken(String);

use std::env;

impl FromRequest for BearerToken {
    type Error = Error;
    type Future = Ready<Result<BearerToken, Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let bearer_token = env::var("BEARER_TOKEN").expect("BEARER_TOKEN must be set");
        if let Some(header) = req.headers().get(http::header::AUTHORIZATION) {
            if let Ok(auth_str) = header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = auth_str[7..].to_string();
                    if token == bearer_token {
                        return ready(Ok(BearerToken(token)));
                    } else {
                        return ready(Err(ErrorUnauthorized("unauthorized")));
                    }
                }
            }
        }

        ready(Err(ErrorUnauthorized("unauthorized")))
    }
}
