use thiserror::Error;
use uuid::Uuid;

/// Domain-level errors — these do not know about HTTP or databases.
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Kit with id {0} not found")]
    KitNotFound(Uuid),

    #[error("Activity with id {0} not found")]
    ActivityNotFound(Uuid),

    #[error("User with id {0} not found")]
    UserNotFound(Uuid),

    #[error("User with email {0} already exists")]
    UserEmailConflict(String),

    #[error("User with username {0} already exists")]
    UserUsernameConflict(String),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Access denied")]
    AccessDenied,

    #[error("Validation error: {0}")]
    Validation(String),
}
