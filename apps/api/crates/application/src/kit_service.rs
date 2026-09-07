use std::sync::Arc;
use uuid::Uuid;

use domain::entities::kit::Kit;
use domain::errors::DomainError;
use domain::repositories::kit_repository::{KitFilters, KitRepository};

use crate::dto::kit::{CreateKitRequest, KitResponse, ListKitsQuery, UpdateKitRequest};

/// Service layer for kit-related business logic.
///
/// This is where use cases live. Handlers call the service;
/// the service calls the repository.
pub struct KitService {
    kit_repo: Arc<dyn KitRepository>,
}

impl KitService {
    pub fn new(kit_repo: Arc<dyn KitRepository>) -> Self {
        Self { kit_repo }
    }

    /// List all published kits with optional filters.
    pub async fn list_kits(&self, query: ListKitsQuery) -> Result<Vec<KitResponse>, DomainError> {
        let filters = KitFilters {
            subject: query.subject,
            age_min: query.age_min,
            age_max: query.age_max,
            difficulty: query.difficulty,
            language: query.language,
            search: query.search,
            page: query.page,
            per_page: query.per_page,
        };

        let kits = self.kit_repo.find_all(filters).await?;
        Ok(kits.into_iter().map(KitResponse::from).collect())
    }

    /// Get a single kit by ID.
    pub async fn get_kit(&self, id: Uuid) -> Result<KitResponse, DomainError> {
        let kit = self.kit_repo.find_by_id(id).await?;
        Ok(KitResponse::from(kit))
    }

    /// Create a new kit for the given author.
    pub async fn create_kit(
        &self,
        request: CreateKitRequest,
        author_id: Uuid,
    ) -> Result<KitResponse, DomainError> {
        // Validate age range
        if request.age_min > request.age_max {
            return Err(DomainError::Validation(
                "age_min must not be greater than age_max".into(),
            ));
        }

        let kit = Kit::new(
            request.title,
            request.description,
            request.subject,
            request.difficulty,
            request.age_min,
            request.age_max,
            request.language,
            request.learning_objectives,
            request.materials_required,
            author_id,
        );

        let created = self.kit_repo.create(kit).await?;
        Ok(KitResponse::from(created))
    }

    /// Update an existing kit. Only the kit's author may do this.
    pub async fn update_kit(
        &self,
        id: Uuid,
        request: UpdateKitRequest,
        requester_id: Uuid,
    ) -> Result<KitResponse, DomainError> {
        let mut kit = self.kit_repo.find_by_id(id).await?;

        if kit.author_id != requester_id {
            return Err(DomainError::AccessDenied);
        }

        if let Some(title) = request.title {
            kit.title = title;
        }
        if let Some(description) = request.description {
            kit.description = description;
        }
        if let Some(subject) = request.subject {
            kit.subject = subject;
        }
        if let Some(difficulty) = request.difficulty {
            kit.difficulty = difficulty;
        }
        if let Some(age_min) = request.age_min {
            kit.age_min = age_min;
        }
        if let Some(age_max) = request.age_max {
            kit.age_max = age_max;
        }
        if let Some(language) = request.language {
            kit.language = language;
        }
        if let Some(objectives) = request.learning_objectives {
            kit.learning_objectives = objectives;
        }
        if let Some(materials) = request.materials_required {
            kit.materials_required = materials;
        }
        if let Some(published) = request.is_published {
            kit.is_published = published;
        }

        let updated = self.kit_repo.update(kit).await?;
        Ok(KitResponse::from(updated))
    }

    /// Delete a kit. Only the kit's author may do this.
    pub async fn delete_kit(&self, id: Uuid, requester_id: Uuid) -> Result<(), DomainError> {
        let kit = self.kit_repo.find_by_id(id).await?;

        if kit.author_id != requester_id {
            return Err(DomainError::AccessDenied);
        }

        self.kit_repo.delete(id).await
    }

    /// Increment the download count for a kit.
    pub async fn record_download(&self, id: Uuid) -> Result<(), DomainError> {
        // Verify kit exists first
        self.kit_repo.find_by_id(id).await?;
        self.kit_repo.increment_download_count(id).await
    }
}
