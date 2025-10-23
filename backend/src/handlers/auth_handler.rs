use actix_web::{web, HttpResponse, HttpRequest};
use serde_json::json;
use actix_web::HttpMessage;


use crate::auth::models::{LoginRequest, ChangePasswordRequest, ResetPasswordRequest, ConfirmResetPasswordRequest};
use crate::auth::service::AuthService;
use crate::repositories::user_repository::UserRepository;
use crate::errors::AppError;

pub struct AuthHandler;

impl AuthHandler {
    pub async fn login(
        pool: web::Data<sqlx::PgPool>,
        login_data: web::Json<LoginRequest>,
    ) -> Result<HttpResponse, AppError> {
        // Trouver l'utilisateur par email
        let user = match UserRepository::find_by_email(&pool, &login_data.email).await {
            Ok(user) => user,
            Err(_) => {
                return Err(AppError::AuthenticationError("Email ou mot de passe incorrect".to_string()));
            }
        };

        // Vérifier si l'utilisateur est actif
        if !user.is_active {
            return Err(AppError::AuthenticationError("Votre compte est désactivé".to_string()));
        }

        // Vérifier le mot de passe
        let is_valid = AuthService::verify_password(&login_data.password, &user.password_hash)?;
        if !is_valid {
            return Err(AppError::AuthenticationError("Email ou mot de passe incorrect".to_string()));
        }

        // Générer le token JWT
        let token = AuthService::generate_token(&user)?;

        // Créer la réponse
        let auth_response = AuthService::create_auth_response(user, token);

        Ok(HttpResponse::Ok().json(auth_response))
    }

    pub async fn logout() -> Result<HttpResponse, AppError> {
        // Pour un logout simple, le client doit simplement supprimer le token
        // Pour un logout plus avancé, on pourrait utiliser une blacklist de tokens
        Ok(HttpResponse::Ok().json(json!({
            "message": "Déconnexion réussie"
        })))
    }

    pub async fn change_password(
        pool: web::Data<sqlx::PgPool>,
        req: HttpRequest,
        password_data: web::Json<ChangePasswordRequest>,
    ) -> Result<HttpResponse, AppError> {
        // Récupérer l'utilisateur depuis le token - CORRECTION ICI
        let extensions = req.extensions();
        let claims = extensions
            .get::<crate::auth::models::Claims>()
            .ok_or_else(|| AppError::AuthenticationError("Token invalide".to_string()))?;

        let user_id = claims.sub;

        // Récupérer l'utilisateur COMPLET avec le mot de passe - CORRECTION ICI
        let user = UserRepository::find_by_id_with_password(&pool, user_id).await?;

        // Vérifier l'ancien mot de passe
        let is_valid = AuthService::verify_password(&password_data.current_password, &user.password_hash)?;
        if !is_valid {
            return Err(AppError::AuthenticationError("Mot de passe actuel incorrect".to_string()));
        }

        // Hasher le nouveau mot de passe
        let new_password_hash = AuthService::hash_password(&password_data.new_password)?;

        // Mettre à jour le mot de passe
        sqlx::query("UPDATE users SET password_hash = $1 WHERE id = $2")
            .bind(&new_password_hash)
            .bind(user_id)
            .execute(&**pool)
            .await?;

        Ok(HttpResponse::Ok().json(json!({
            "message": "Mot de passe modifié avec succès"
        })))
    }

    pub async fn get_me(
        pool: web::Data<sqlx::PgPool>,
        req: HttpRequest,
    ) -> Result<HttpResponse, AppError> {
        // Récupérer l'utilisateur depuis le token - CORRECTION ICI
        let extensions = req.extensions_mut();
        let claims = extensions
            .get::<crate::auth::models::Claims>()
            .ok_or_else(|| AppError::AuthenticationError("Token invalide".to_string()))?;

        let user_id = claims.sub;

        // Récupérer l'utilisateur (version publique sans mot de passe)
        let user = UserRepository::find_by_id(&pool, user_id).await?;

        Ok(HttpResponse::Ok().json(user))
    }

    pub async fn refresh_token(
        pool: web::Data<sqlx::PgPool>,
        req: HttpRequest,
    ) -> Result<HttpResponse, AppError> {
        // Récupérer l'utilisateur depuis le token - CORRECTION ICI
        let extensions = req.extensions();
        let claims = extensions
            .get::<crate::auth::models::Claims>()
            .ok_or_else(|| AppError::AuthenticationError("Token invalide".to_string()))?;

        let user_id = claims.sub;

        // Récupérer l'utilisateur COMPLET pour générer le token - CORRECTION ICI
        let user = UserRepository::find_by_id_with_password(&pool, user_id).await?;

        // Générer un nouveau token
        let new_token = AuthService::generate_token(&user)?;

        // Créer la réponse
        let auth_response = AuthService::create_auth_response(user, new_token);

        Ok(HttpResponse::Ok().json(auth_response))
    }

    pub async fn request_password_reset(
        pool: web::Data<sqlx::PgPool>,
        reset_data: web::Json<ResetPasswordRequest>,
    ) -> Result<HttpResponse, AppError> {
        // Dans une vraie application, on enverrait un email avec un lien de réinitialisation
        // Pour l'instant, on simule juste l'envoi
        let _user = match UserRepository::find_by_email(&pool, &reset_data.email).await {
            Ok(user) => user,
            Err(_) => {
                // On ne révèle pas si l'email existe ou pas
                return Ok(HttpResponse::Ok().json(json!({
                    "message": "Si l'email existe, un lien de réinitialisation a été envoyé"
                })));
            }
        };

        // Ici, on générerait un token de réinitialisation et on enverrait un email
        // Pour la démo, on retourne juste un message

        Ok(HttpResponse::Ok().json(json!({
            "message": "Si l'email existe, un lien de réinitialisation a été envoyé"
        })))
    }

    pub async fn confirm_password_reset(
        pool: web::Data<sqlx::PgPool>,
        confirm_data: web::Json<ConfirmResetPasswordRequest>,
    ) -> Result<HttpResponse, AppError> {
        // Dans une vraie application, on vérifierait le token de réinitialisation
        // Pour la démo, on simule la réinitialisation

        // Ici, on devrait avoir une table pour stocker les tokens de réinitialisation
        // et vérifier leur validité

        // Pour l'instant, on retourne une erreur indiquant que c'est une démo
        Err(AppError::ValidationError("Fonctionnalité de réinitialisation en cours de développement".to_string()))
    }
}