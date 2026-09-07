use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use domain::entities::user::{User, UserRole};
use domain::errors::DomainError;
use domain::repositories::user_repository::UserRepository;

use crate::dto::auth::{AuthResponse, LoginRequest, RegisterRequest, UserSummary};

/// JWT claims payload.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject — the user's UUID.
    pub sub: String,
    /// Issued at (Unix timestamp).
    pub iat: i64,
    /// Expiry (Unix timestamp).
    pub exp: i64,
}

pub struct AuthService {
    user_repo: Arc<dyn UserRepository>,
    jwt_secret: String,
    jwt_expiration_hours: u64,
}

impl AuthService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        jwt_secret: String,
        jwt_expiration_hours: u64,
    ) -> Self {
        Self {
            user_repo,
            jwt_secret,
            jwt_expiration_hours,
        }
    }

    /// Register a new user.
    pub async fn register(&self, request: RegisterRequest) -> Result<AuthResponse, DomainError> {
        // Hash the password with Argon2
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(request.password.as_bytes(), &salt)
            .map_err(|e| DomainError::Validation(format!("Password hashing failed: {e}")))?
            .to_string();

        let now = Utc::now();
        let user = User {
            id: Uuid::new_v4(),
            email: request.email,
            username: request.username,
            password_hash,
            display_name: request.display_name,
            bio: None,
            role: UserRole::Contributor,
            is_active: true,
            created_at: now,
            updated_at: now,
        };

        let created_user = self.user_repo.create(user).await?;
        let token = self.generate_token(&created_user)?;

        Ok(AuthResponse {
            access_token: token,
            token_type: "Bearer".into(),
            expires_in: self.jwt_expiration_hours * 3600,
            user: UserSummary {
                id: created_user.id.to_string(),
                email: created_user.email,
                username: created_user.username,
                display_name: created_user.display_name,
            },
        })
    }

    /// Authenticate a user and return a JWT.
    pub async fn login(&self, request: LoginRequest) -> Result<AuthResponse, DomainError> {
        let user = self
            .user_repo
            .find_by_email(&request.email)
            .await
            .map_err(|_| DomainError::InvalidCredentials)?;

        if !user.is_active {
            return Err(DomainError::InvalidCredentials);
        }

        // Verify password
        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|_| DomainError::InvalidCredentials)?;

        Argon2::default()
            .verify_password(request.password.as_bytes(), &parsed_hash)
            .map_err(|_| DomainError::InvalidCredentials)?;

        let token = self.generate_token(&user)?;

        Ok(AuthResponse {
            access_token: token,
            token_type: "Bearer".into(),
            expires_in: self.jwt_expiration_hours * 3600,
            user: UserSummary {
                id: user.id.to_string(),
                email: user.email,
                username: user.username,
                display_name: user.display_name,
            },
        })
    }

    /// Validate a JWT and return the user's UUID.
    pub fn validate_token(&self, token: &str) -> Result<Uuid, DomainError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| DomainError::InvalidCredentials)?;

        Uuid::parse_str(&token_data.claims.sub)
            .map_err(|_| DomainError::InvalidCredentials)
    }

    fn generate_token(&self, user: &User) -> Result<String, DomainError> {
        let now = Utc::now().timestamp();
        let expiry = now + (self.jwt_expiration_hours as i64 * 3600);

        let claims = Claims {
            sub: user.id.to_string(),
            iat: now,
            exp: expiry,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| DomainError::Validation(format!("Token generation failed: {e}")))
    }
}
