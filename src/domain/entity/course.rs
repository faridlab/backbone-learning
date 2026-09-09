use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::AuditMetadata;
use super::CourseFormat;
use super::CourseStatus;

/// Strongly-typed ID for Course
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CourseId(pub Uuid);

impl CourseId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }
    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl std::fmt::Display for CourseId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for CourseId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for CourseId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<CourseId> for Uuid {
    fn from(id: CourseId) -> Self {
        id.0
    }
}

impl AsRef<Uuid> for CourseId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl std::ops::Deref for CourseId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Course {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub format: CourseFormat,
    pub duration_hours: Option<Decimal>,
    pub cost: Option<Decimal>,
    pub provider: Option<String>,
    pub certification_id: Option<Uuid>,
    pub status: CourseStatus,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl Course {
    /// Create a builder for Course
    pub fn builder() -> CourseBuilder {
        CourseBuilder::default()
    }

    /// Create a new Course with required fields
    pub fn new(name: String, format: CourseFormat, status: CourseStatus) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description: None,
            format,
            duration_hours: None,
            cost: None,
            provider: None,
            certification_id: None,
            status,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> CourseId {
        CourseId(self.id)
    }

    /// Get when this entity was created
    pub fn created_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.created_at.as_ref()
    }

    /// Get when this entity was last updated
    pub fn updated_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.updated_at.as_ref()
    }

    /// Check if this entity is soft deleted
    pub fn is_deleted(&self) -> bool {
        self.metadata.deleted_at.is_some()
    }

    /// Check if this entity is active (not deleted)
    pub fn is_active(&self) -> bool {
        self.metadata.deleted_at.is_none()
    }

    /// Get when this entity was deleted
    pub fn deleted_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.deleted_at.as_ref()
    }

    /// Get who created this entity
    pub fn created_by(&self) -> Option<&Uuid> {
        self.metadata.created_by.as_ref()
    }

    /// Get who last updated this entity
    pub fn updated_by(&self) -> Option<&Uuid> {
        self.metadata.updated_by.as_ref()
    }

    /// Get who deleted this entity
    pub fn deleted_by(&self) -> Option<&Uuid> {
        self.metadata.deleted_by.as_ref()
    }

    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the description field (chainable)
    pub fn with_description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    /// Set the duration_hours field (chainable)
    pub fn with_duration_hours(mut self, value: Decimal) -> Self {
        self.duration_hours = Some(value);
        self
    }

    /// Set the cost field (chainable)
    pub fn with_cost(mut self, value: Decimal) -> Self {
        self.cost = Some(value);
        self
    }

    /// Set the provider field (chainable)
    pub fn with_provider(mut self, value: String) -> Self {
        self.provider = Some(value);
        self
    }

    /// Set the certification_id field (chainable)
    pub fn with_certification_id(mut self, value: Uuid) -> Self {
        self.certification_id = Some(value);
        self
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "name" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.name = v;
                    }
                }
                "description" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.description = v;
                    }
                }
                "format" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.format = v;
                    }
                }
                "duration_hours" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.duration_hours = v;
                    }
                }
                "cost" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.cost = v;
                    }
                }
                "provider" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.provider = v;
                    }
                }
                "certification_id" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.certification_id = v;
                    }
                }
                "status" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.status = v;
                    }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for Course {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "Course"
    }
}

impl backbone_core::PersistentEntity for Course {
    fn entity_id(&self) -> String {
        self.id.to_string()
    }
    fn set_entity_id(&mut self, id: String) {
        if let Ok(uuid) = uuid::Uuid::parse_str(&id) {
            self.id = uuid;
        }
    }
    fn created_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.created_at
    }
    fn set_created_at(&mut self, ts: chrono::DateTime<chrono::Utc>) {
        self.metadata.created_at = Some(ts);
    }
    fn updated_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.updated_at
    }
    fn set_updated_at(&mut self, ts: chrono::DateTime<chrono::Utc>) {
        self.metadata.updated_at = Some(ts);
    }
    fn deleted_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.deleted_at
    }
    fn set_deleted_at(&mut self, ts: Option<chrono::DateTime<chrono::Utc>>) {
        self.metadata.deleted_at = ts;
    }
}

impl backbone_orm::EntityRepoMeta for Course {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("certification_id".to_string(), "uuid".to_string());
        m.insert("format".to_string(), "course_format".to_string());
        m.insert("status".to_string(), "course_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["name"]
    }
}

/// Builder for Course entity
///
/// Provides a fluent API for constructing Course instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct CourseBuilder {
    name: Option<String>,
    description: Option<String>,
    format: Option<CourseFormat>,
    duration_hours: Option<Decimal>,
    cost: Option<Decimal>,
    provider: Option<String>,
    certification_id: Option<Uuid>,
    status: Option<CourseStatus>,
}

impl CourseBuilder {
    /// Set the name field (required)
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    /// Set the description field (optional)
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    /// Set the format field (default: `CourseFormat::default()`)
    pub fn format(mut self, value: CourseFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Set the duration_hours field (optional)
    pub fn duration_hours(mut self, value: Decimal) -> Self {
        self.duration_hours = Some(value);
        self
    }

    /// Set the cost field (optional)
    pub fn cost(mut self, value: Decimal) -> Self {
        self.cost = Some(value);
        self
    }

    /// Set the provider field (optional)
    pub fn provider(mut self, value: String) -> Self {
        self.provider = Some(value);
        self
    }

    /// Set the certification_id field (optional)
    pub fn certification_id(mut self, value: Uuid) -> Self {
        self.certification_id = Some(value);
        self
    }

    /// Set the status field (default: `CourseStatus::default()`)
    pub fn status(mut self, value: CourseStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Build the Course entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<Course, String> {
        let name = self.name.ok_or_else(|| "name is required".to_string())?;

        Ok(Course {
            id: Uuid::new_v4(),
            name,
            description: self.description,
            format: self.format.unwrap_or(CourseFormat::default()),
            duration_hours: self.duration_hours,
            cost: self.cost,
            provider: self.provider,
            certification_id: self.certification_id,
            status: self.status.unwrap_or(CourseStatus::default()),
            metadata: AuditMetadata::default(),
        })
    }
}
