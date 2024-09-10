use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use reqwest::ClientBuilder;
use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub exp: u64,
    pub iat: u64,
    pub auth_time: u64,
    pub acr: String,
    pub azp: String,
    #[serde(rename="uid")]
    pub nickname: String,
    // pub amr: Vec<String>,
    // pub email: String,
    // pub email_verified: bool,
    // pub name: String,
    // pub given_name: String,
    // pub preferred_username: String,
    // pub nickname: String,
    // pub groups: Vec<String>,
}

#[derive(Deserialize, Default, Debug)]
pub struct KeyResult {
    pub kty: String,
    #[serde(rename = "use")]
    pub usage: String,
    pub n: String,
    pub e: String,
    pub kid: String,
    pub x5t: String,
    pub alg: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct KeysResult {
    pub keys: Vec<KeyResult>,
}

pub async fn get_userinfo(
    access_token: &str, 
    userinfo_url: &str
) -> anyhow::Result<Claims> {
    let client = ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    let response = client
        .get(userinfo_url)
        .bearer_auth(access_token)
        .send()
        .await?;

    response.json::<Claims>()
        .await
        .map_err(|req_err| {
            anyhow::anyhow!("Failed to parse claims: {}", req_err)
        })
}

pub async fn validate_token(
    access_token: &str, 
    jwks_url: &str
) -> anyhow::Result<TokenData<Claims>> {
    let client = ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    let validation = Validation::new(Algorithm::RS256);

    let response = client
        .get(jwks_url)
        .bearer_auth(access_token)
        .send()
        .await?;

    let des_result = response.json::<KeysResult>().await?;

    let n = &des_result.keys[0].n;
    let e = &des_result.keys[0].e;
    let decoding_key = DecodingKey::from_rsa_components(n, e)?;
    decode::<Claims>(
        access_token,
        &decoding_key,
        &validation,
    ).map_err(|dec_err| {
        anyhow::anyhow!("Failed to validate token: {}", dec_err)
    })
}