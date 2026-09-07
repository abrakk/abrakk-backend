use serde::{Deserialize, Serialize};
use validator::Validate;

use domain::entities::kit::{DifficultyLevel, SubjectArea};

/// Request body for creating a new kit.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateKitRequest {
    #[validate(length(min = 3, max = 200, message = "Title must be between 3 and 200 characters"))]
    pub title: String,

    #[validate(length(min = 10, message = "Description must be at least 10 characters"))]
    pub description: String,

    pub subject: SubjectArea,
    pub difficulty: DifficultyLevel,

    #[validate(range(min = 0, max = 18, message = "Age must be between 0 and 18"))]
    pub age_min: i32,

    #[validate(range(min = 0, max = 18, message = "Age must be between 0 and 18"))]
    pub age_max: i32,

    #[validate(length(min = 2, max = 10, message = "Language code must be valid"))]
    pub language: String,

    #[validate(length(min = 1, message = "At least one learning objective is required"))]
    pub learning_objectives: Vec<String>,

    pub materials_required: Vec<String>,
}

/// Request body for updating a kit.
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateKitRequest {
    #[validate(length(min = 3, max = 200))]
    pub title: Option<String>,

    #[validate(length(min = 10))]
    pub description: Option<String>,

    pub subject: Option<SubjectArea>,
    pub difficulty: Option<DifficultyLevel>,
    pub age_min: Option<i32>,
    pub age_max: Option<i32>,
    pub language: Option<String>,
    pub learning_objectives: Option<Vec<String>>,
    pub materials_required: Option<Vec<String>>,
    pub is_published: Option<bool>,
}

/// Query parameters for listing kits.
#[derive(Debug, Deserialize)]
pub struct ListKitsQuery {
    pub subject: Option<String>,
    pub age_min: Option<i32>,
    pub age_max: Option<i32>,
    pub difficulty: Option<String>,
    pub language: Option<String>,
    pub search: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

/// Kit response sent to API consumers.
#[derive(Debug, Serialize)]
pub struct KitResponse {
    pub id: String,
    pub title: String,
    pub description: String,
    pub subject: SubjectArea,
    pub difficulty: DifficultyLevel,
    pub age_min: i32,
    pub age_max: i32,
    pub language: String,
    pub learning_objectives: Vec<String>,
    pub materials_required: Vec<String>,
    pub author_id: String,
    pub is_published: bool,
    pub download_count: i64,
    pub created_at: String,
    pub updated_at: String,
}

impl From<domain::entities::kit::Kit> for KitResponse {
    fn from(kit: domain::entities::kit::Kit) -> Self {
        KitResponse {
            id: kit.id.to_string(),
            title: kit.title,
            description: kit.description,
            subject: kit.subject,
            difficulty: kit.difficulty,
            age_min: kit.age_min,
            age_max: kit.age_max,
            language: kit.language,
            learning_objectives: kit.learning_objectives,
            materials_required: kit.materials_required,
            author_id: kit.author_id.to_string(),
            is_published: kit.is_published,
            download_count: kit.download_count,
            created_at: kit.created_at.to_rfc3339(),
            updated_at: kit.updated_at.to_rfc3339(),
        }
    }
}
