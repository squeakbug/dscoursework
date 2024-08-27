use std::future::{ready, Ready};

use actix_web::{Error, FromRequest, HttpRequest};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub ip: String,
}

pub struct JwtValidator {
    secret: String,
}

impl JwtValidator {
    pub fn new(secret: &str) -> Self {
        JwtValidator {
            secret: secret.to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct JwtAuthGuard {
    pub token: String,
    pub claims: Claims,
}

impl FromRequest for JwtAuthGuard {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let secret = req
            .app_data::<JwtValidator>()
            .map(|data| data.secret.clone())
            .unwrap_or_else(|| "".to_string());

        let token = req
            .headers()
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .map(|token| if token.starts_with("Bearer ") {
                &token[7..]
            } else {
                token
            });

        if let Some(token) = token {
            let decoding_key = DecodingKey::from_secret(secret.as_ref());
            let mut validation = Validation::new(Algorithm::HS256);
            validation.leeway = 60;
    
            let token_data = decode::<Claims>(token, &decoding_key, &validation);
    
            if let Ok(token_data) = token_data {
                let client_ip = req
                    .connection_info()
                    .peer_addr()
                    .unwrap_or_default()
                    .to_string();
    
                if token_data.claims.ip == client_ip {
                    return ready(Ok(JwtAuthGuard {
                        claims: token_data.claims,
                        token: token.to_owned(),
                    }));
                }
            }
        }

        
        ready(Err(actix_web::error::ErrorUnauthorized("")))
    }
}
