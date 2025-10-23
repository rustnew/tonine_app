use std::env;
use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;

pub struct JwtConfig {
    pub secret: String,
    pub expiration: i64,
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
}

pub static JWT_CONFIG: Lazy<JwtConfig> = Lazy::new(|| {
    let secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");
    let expiration = env::var("JWT_EXPIRATION")
        .unwrap_or_else(|_| "86400".to_string()) // 24 heures par défaut
        .parse()
        .expect("JWT_EXPIRATION must be a number");

    JwtConfig {
        secret: secret.clone(),
        expiration,
        encoding_key: EncodingKey::from_secret(secret.as_ref()),
        decoding_key: DecodingKey::from_secret(secret.as_ref()),
    }
});