use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use domain::entities::kit::Kit;
use domain::errors::DomainError;
use domain::repositories::kit_repository::{KitFilters, KitRepository};

pub struct PostgresKitRepository {
    pool: PgPool,
}

impl PostgresKitRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl KitRepository for PostgresKitRepository {
    async fn find_all(&self, filters: KitFilters) -> Result<Vec<Kit>, DomainError> {
        let page = filters.page.unwrap_or(1).max(1);
        let per_page = filters.per_page.unwrap_or(20).clamp(1, 100);
        let offset = (page - 1) * per_page;

        // Build a dynamic query using sqlx query_as
        // A full implementation would use a query builder; this uses a base query.
        let kits = sqlx::query_as!(
            KitRow,
            r#"
            SELECT
                id, title, description,
                subject AS "subject: _",
                difficulty AS "difficulty: _",
                age_min, age_max, language,
                learning_objectives, materials_required,
                author_id, is_published, download_count,
                created_at, updated_at
            FROM kits
            WHERE is_published = true
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            per_page,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch kits: {e}");
            DomainError::Validation(e.to_string())
        })?;

        Ok(kits.into_iter().map(Into::into).collect())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Kit, DomainError> {
        let kit = sqlx::query_as!(
            KitRow,
            r#"
            SELECT
                id, title, description,
                subject AS "subject: _",
                difficulty AS "difficulty: _",
                age_min, age_max, language,
                learning_objectives, materials_required,
                author_id, is_published, download_count,
                created_at, updated_at
            FROM kits
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Validation(e.to_string()))?
        .ok_or(DomainError::KitNotFound(id))?;

        Ok(kit.into())
    }

    async fn create(&self, kit: Kit) -> Result<Kit, DomainError> {
        sqlx::query!(
            r#"
            INSERT INTO kits (
                id, title, description, subject, difficulty,
                age_min, age_max, language, learning_objectives,
                materials_required, author_id, is_published,
                download_count, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            "#,
            kit.id,
            kit.title,
            kit.description,
            kit.subject as _,
            kit.difficulty as _,
            kit.age_min,
            kit.age_max,
            kit.language,
            &kit.learning_objectives,
            &kit.materials_required,
            kit.author_id,
            kit.is_published,
            kit.download_count,
            kit.created_at,
            kit.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::Validation(e.to_string()))?;

        Ok(kit)
    }

    async fn update(&self, kit: Kit) -> Result<Kit, DomainError> {
        let updated = sqlx::query!(
            r#"
            UPDATE kits
            SET title = $2, description = $3, subject = $4,
                difficulty = $5, age_min = $6, age_max = $7,
                language = $8, learning_objectives = $9,
                materials_required = $10, is_published = $11,
                updated_at = NOW()
            WHERE id = $1
            RETURNING updated_at
            "#,
            kit.id,
            kit.title,
            kit.description,
            kit.subject as _,
            kit.difficulty as _,
            kit.age_min,
            kit.age_max,
            kit.language,
            &kit.learning_objectives,
            &kit.materials_required,
            kit.is_published,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Validation(e.to_string()))?
        .ok_or(DomainError::KitNotFound(kit.id))?;

        Ok(Kit {
            updated_at: updated.updated_at,
            ..kit
        })
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        let result = sqlx::query!("DELETE FROM kits WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::Validation(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(DomainError::KitNotFound(id));
        }

        Ok(())
    }

    async fn increment_download_count(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query!(
            "UPDATE kits SET download_count = download_count + 1 WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::Validation(e.to_string()))?;

        Ok(())
    }
}

// ── Internal row type for SQLx ────────────────────────────────────────────────

use domain::entities::kit::{DifficultyLevel, SubjectArea};

struct KitRow {
    id: Uuid,
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
    is_published: bool,
    download_count: i64,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<KitRow> for Kit {
    fn from(row: KitRow) -> Self {
        Kit {
            id: row.id,
            title: row.title,
            description: row.description,
            subject: row.subject,
            difficulty: row.difficulty,
            age_min: row.age_min,
            age_max: row.age_max,
            language: row.language,
            learning_objectives: row.learning_objectives,
            materials_required: row.materials_required,
            author_id: row.author_id,
            is_published: row.is_published,
            download_count: row.download_count,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
