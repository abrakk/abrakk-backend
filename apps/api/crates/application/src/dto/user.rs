use serde::{Deserialize, Serialize};
use validator::Validate;

use domain::entities::user::UserRole;

/// Public user profile response.
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub username: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub role: UserRole,
    pub created_at: String,
}

impl From<domain::entities::user::User> for UserResponse {
    fn from(user: domain::entities::user::User) -> Self {
        UserResponse {
            id: user.id.to_string(),
            email: user.email,
            username: user.username,
            display_name: user.display_name,
            bio: user.bio,
            role: user.role,
            created_at: user.created_at.to_rfc3339(),
        }
    }
}

/// Update profile request.
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfileRequest {
    #[validate(length(min = 2, max = 100))]
    pub display_name: Option<String>,

    #[validate(length(max = 500))]
    pub bio: Option<String>,
}
