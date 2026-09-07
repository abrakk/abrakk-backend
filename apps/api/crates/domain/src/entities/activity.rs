use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A single learning activity that belongs to a Kit.
///
/// Activities are the individual tasks or exercises within a kit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub id: Uuid,
    pub kit_id: Uuid,
    pub title: String,
    pub description: String,
    pub instructions: String,
    pub duration_minutes: i32,
    pub order_index: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Activity {
    pub fn new(
        kit_id: Uuid,
        title: String,
        description: String,
        instructions: String,
        duration_minutes: i32,
        order_index: i32,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            kit_id,
            title,
            description,
            instructions,
            duration_minutes,
            order_index,
            created_at: now,
            updated_at: now,
        }
    }
}
