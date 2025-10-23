use jsonwebtoken::{encode, decode, Header, Validation};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Utc, Duration};


use crate::auth::models::{Claims, AuthResponse, UserAuthResponse};
use crate::auth::config::JWT_CONFIG;
use crate::model::users::User;
use crate::errors::AppError;

pub struct AuthService;

impl AuthService {

    pub fn generate_token(user: &User) -> Result<String, AppError> {
        let now = Utc::now();
        let expiration = now + Duration::seconds(JWT_CONFIG.expiration);

        let claims = Claims {
            sub: user.id,
            email: user.email.clone(),
            exp: expiration.timestamp(),
            iat: now.timestamp(),
        };

        encode(&Header::default(), &claims, &JWT_CONFIG.encoding_key)
            .map_err(|e| AppError::AuthenticationError(e.to_string()))
    }

    pub fn verify_token(token: &str) -> Result<Claims, AppError> {
        let validation = Validation::default();
        
        decode::<Claims>(token, &JWT_CONFIG.decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|e| AppError::AuthenticationError(e.to_string()))
    }

    pub fn hash_password(password: &str) -> Result<String, AppError> {
        hash(password, DEFAULT_COST)
            .map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
        verify(password, hash)
            .map_err(|e| AppError::AuthenticationError(e.to_string()))
    }

    
    pub fn create_auth_response(user: User, token: String) -> AuthResponse {
        AuthResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: JWT_CONFIG.expiration,
            user: UserAuthResponse {
                id: user.id,
                email: user.email,
                phone: user.phone,
                full_name: user.full_name,
                is_active: user.is_active,
            },
        }
    }
}