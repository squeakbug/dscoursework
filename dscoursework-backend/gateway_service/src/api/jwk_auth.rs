use std::pin::Pin;

use actix_web::{
    dev::Payload,
    error::{Error as ActixWebError, ErrorUnauthorized},
    http, web, FromRequest, HttpRequest,
};
use futures::Future;
use serde_json::json;

use shared::auth::authentik_auth::{Claims, validate_token};
use crate::state::AppState;

pub struct AuthenticationGuard {
    pub claims: Claims,
    pub token: String,
}

impl FromRequest for AuthenticationGuard {
    type Error = ActixWebError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let access_token = req.headers()
            .get(http::header::AUTHORIZATION)
            .map(|h| {
                h.to_str().unwrap().split_at(7).1.to_string()
            });

        if access_token.is_none() {
            return Box::pin(async move {
                Err(ErrorUnauthorized(
                    json!({
                        "status": "fail", 
                        "message": "Token was not provided"
                    }),
                ))
            });
        }
        let access_token = access_token.unwrap();

        let data = req.app_data::<web::Data<AppState>>().unwrap().clone();
        Box::pin(async move {
            let decode = validate_token(
                &access_token,
                &data.config.authentik_jwks,
            ).await;

            match decode {
                Ok(decode) => {
                    Ok(AuthenticationGuard {
                        claims: decode.claims,
                        token: access_token,
                    })
                }
                Err(_) => Err(ErrorUnauthorized(
                    json!({
                        "status": "fail", 
                        "message": "Invalid token or usre doesn't exists"
                    }),
                )),
            }
        })
    }
}