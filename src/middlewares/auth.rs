use crate::entities::ResponseWrapper;
use crate::helpers::jwt::{decode_jwt, TokenUser};
use actix_web::{error::InternalError, http::header, FromRequest, HttpResponse};
use std::future::{ready, Ready};

pub struct Auth(pub TokenUser);

impl FromRequest for Auth {
    type Error = InternalError<String>;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let access_token = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|str| str.split(" ").nth(1));

        match access_token {
            Some(token) => {
                let user = decode_jwt(token);

                match user {
                    Ok(user) => ready(Ok(Auth(user))),
                    Err(err) => ready(Err(InternalError::from_response(
                        err.clone(),
                        HttpResponse::Unauthorized().json(ResponseWrapper::<()> {
                            message: err,
                            data: None,
                        }),
                    ))),
                }
            }
            None => ready(Err(InternalError::from_response(
                "Unauthorized".to_string(),
                HttpResponse::Unauthorized().json(ResponseWrapper::<()> {
                    message: "Unauthorized".to_string(),
                    data: None,
                }),
            ))),
        }
    }
}
