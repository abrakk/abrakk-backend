use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Difficulty level of a learning kit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "difficulty_level", rename_all = "lowercase")]
pub enum DifficultyLevel {
    Beginner,
    Intermediate,
    Advanced,
}

/// Subject area of a learning kit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "subject_area", rename_all = "snake_case")]
pub enum SubjectArea {
    Mathematics,
    Literacy,
    Science,
    Arts,
    SocialStudies,
    Languages,
    PhysicalEducation,
    Other,
}

/// A learning kit — the core resource on the platform.
///
/// A kit groups together educational activities, resources, and materials
/// around a specific topic, age group, and subject area.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kit {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub subject: SubjectArea,
    pub difficulty: DifficultyLevel,
    pub age_min: i32,
    pub age_max: i32,
    pub language: String,
    pub learning_objectives: Vec<String>,
    pub materials_required: Vec<String>,
    pub author_id: Uuid,
    pub is_published: bool,
    pub download_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Kit {
    /// Create a new unpublished kit.
    pub fn new(
        title: String,
        description: String,
        subject: SubjectArea,
        difficulty: DifficultyLevel,
        age_min: i32,
        age_max: i32,
        language: String,
        learning_objectives: Vec<String>,
        materials_required: Vec<String>,
        author_id: Uuid,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            subject,
            difficulty,
            age_min,
            age_max,
            language,
            learning_objectives,
            materials_required,
            author_id,
            is_published: false,
            download_count: 0,
            created_at: now,
            updated_at: now,
        }
    }
}
