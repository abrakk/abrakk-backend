use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use domain::entities::user::{User, UserRole};
use domain::errors::DomainError;
use domain::repositories::user_repository::UserRepository;

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<User, DomainError> {
        let row = sqlx::query_as!(
            UserRow,
            r#"
            SELECT id, email, username, password_hash,
                   display_name, bio, role AS "role: _",
                   is_active, created_at, updated_at
            FROM users WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Validation(e.to_string()))?
        .ok_or(DomainError::UserNotFound(id))?;

        Ok(row.into())
    }

    async fn find_by_email(&self, email: &str) -> Result<User, DomainError> {
        let row = sqlx::query_as!(
            UserRow,
            r#"
            SELECT id, email, username, password_hash,
                   display_name, bio, role AS "role: _",
                   is_active, created_at, updated_at
            FROM users WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Validation(e.to_string()))?
        .ok_or_else(|| DomainError::Validation(format!("User with email {email} not found")))?;

        Ok(row.into())
    }

    async fn find_by_username(&self, username: &str) -> Result<User, DomainError> {
        let row = sqlx::query_as!(
            UserRow,
            r#"
            SELECT id, email, username, password_hash,
                   display_name, bio, role AS "role: _",
                   is_active, created_at, updated_at
            FROM users WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Validation(e.to_string()))?
        .ok_or_else(|| {
            DomainError::Validation(format!("User with username {username} not found"))
        })?;

        Ok(row.into())
    }

    async fn create(&self, user: User) -> Result<User, DomainError> {
        sqlx::query!(
            r#"
            INSERT INTO users (
                id, email, username, password_hash,
                display_name, bio, role, is_active,
                created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            user.id,
            user.email,
            user.username,
            user.password_hash,
            user.display_name,
            user.bio,
            user.role as _,
            user.is_active,
            user.created_at,
            user.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            // Check for unique constraint violations
            if e.to_string().contains("users_email_key") {
                DomainError::UserEmailConflict(user.email.clone())
            } else if e.to_string().contains("users_username_key") {
                DomainError::UserUsernameConflict(user.username.clone())
            } else {
                DomainError::Validation(e.to_string())
            }
        })?;

        Ok(user)
    }

    async fn update(&self, user: User) -> Result<User, DomainError> {
        let updated = sqlx::query!(
            r#"
            UPDATE users
            SET display_name = $2, bio = $3, updated_at = NOW()
            WHERE id = $1
            RETURNING updated_at
            "#,
            user.id,
            user.display_name,
            user.bio,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Validation(e.to_string()))?
        .ok_or(DomainError::UserNotFound(user.id))?;

        Ok(User {
            updated_at: updated.updated_at,
            ..user
        })
    }
}

// ── Internal row type ─────────────────────────────────────────────────────────

struct UserRow {
    id: Uuid,
    email: String,
    username: String,
    password_hash: String,
    display_name: String,
    bio: Option<String>,
    role: UserRole,
    is_active: bool,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        User {
            id: row.id,
            email: row.email,
            username: row.username,
            password_hash: row.password_hash,
            display_name: row.display_name,
            bio: row.bio,
            role: row.role,
            is_active: row.is_active,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
