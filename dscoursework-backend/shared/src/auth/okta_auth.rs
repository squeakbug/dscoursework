use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Default, Debug)]
pub struct Claims {
    pub sub: String,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub preferred_username: String,
    pub nickname: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
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

pub async fn parse_claims(
    access_token: &str, 
    domain_string: &str
) -> anyhow::Result<Claims> {
    let client = Client::new();

    let local_var_uri_str = format!("https://{0}/v1/userinfo", domain_string);
    let uri_str = Url::parse(&local_var_uri_str)?;

    let response = client
        .get(uri_str)
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
    audience: &str,
    well_known_url: &str
) -> anyhow::Result<TokenData<TokenClaims>> {
    let client = Client::new();

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[audience]);

    let local_var_uri_str = well_known_url.to_owned();
    let uri_str = Url::parse(&local_var_uri_str)?;

    let response = client
        .get(uri_str)
        .bearer_auth(access_token)
        .send()
        .await?;
    let des_result = response.json::<KeysResult>().await?;

    let n = &des_result.keys[0].n;
    let e = &des_result.keys[0].e;
    let decoding_key = DecodingKey::from_rsa_components(n, e)?;
    decode::<TokenClaims>(
        access_token,
        &decoding_key,
        &validation,
    ).map_err(|dec_err| {
        anyhow::anyhow!("Failed to validate token: {}", dec_err)
    })
}