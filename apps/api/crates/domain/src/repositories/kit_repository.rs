use async_trait::async_trait;
use uuid::Uuid;

use crate::entities::kit::Kit;
use crate::errors::DomainError;

/// Filter options for listing kits.
#[derive(Debug, Default)]
pub struct KitFilters {
    pub subject: Option<String>,
    pub age_min: Option<i32>,
    pub age_max: Option<i32>,
    pub difficulty: Option<String>,
    pub language: Option<String>,
    pub search: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

/// Repository trait for Kit persistence.
///
/// Implementations live in the infrastructure crate. This keeps the domain
/// free from database concerns.
#[async_trait]
pub trait KitRepository: Send + Sync {
    async fn find_all(&self, filters: KitFilters) -> Result<Vec<Kit>, DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Kit, DomainError>;
    async fn create(&self, kit: Kit) -> Result<Kit, DomainError>;
    async fn update(&self, kit: Kit) -> Result<Kit, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
    async fn increment_download_count(&self, id: Uuid) -> Result<(), DomainError>;
}
