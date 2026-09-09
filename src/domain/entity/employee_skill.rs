use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::AuditMetadata;
use super::ProficiencyLevel;
use super::SkillVerification;

/// Strongly-typed ID for EmployeeSkill
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EmployeeSkillId(pub Uuid);

impl EmployeeSkillId {
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

impl std::fmt::Display for EmployeeSkillId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for EmployeeSkillId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for EmployeeSkillId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<EmployeeSkillId> for Uuid {
    fn from(id: EmployeeSkillId) -> Self {
        id.0
    }
}

impl AsRef<Uuid> for EmployeeSkillId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl std::ops::Deref for EmployeeSkillId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EmployeeSkill {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub skill_id: Uuid,
    pub proficiency: ProficiencyLevel,
    pub years_experience: Option<Decimal>,
    pub verification: SkillVerification,
    pub verified_by: Option<Uuid>,
    pub last_used_at: Option<NaiveDate>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl EmployeeSkill {
    /// Create a builder for EmployeeSkill
    pub fn builder() -> EmployeeSkillBuilder {
        EmployeeSkillBuilder::default()
    }

    /// Create a new EmployeeSkill with required fields
    pub fn new(
        employee_id: Uuid,
        skill_id: Uuid,
        proficiency: ProficiencyLevel,
        verification: SkillVerification,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            employee_id,
            skill_id,
            proficiency,
            years_experience: None,
            verification,
            verified_by: None,
            last_used_at: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> EmployeeSkillId {
        EmployeeSkillId(self.id)
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

    /// Set the years_experience field (chainable)
    pub fn with_years_experience(mut self, value: Decimal) -> Self {
        self.years_experience = Some(value);
        self
    }

    /// Set the verified_by field (chainable)
    pub fn with_verified_by(mut self, value: Uuid) -> Self {
        self.verified_by = Some(value);
        self
    }

    /// Set the last_used_at field (chainable)
    pub fn with_last_used_at(mut self, value: NaiveDate) -> Self {
        self.last_used_at = Some(value);
        self
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "employee_id" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.employee_id = v;
                    }
                }
                "skill_id" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.skill_id = v;
                    }
                }
                "proficiency" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.proficiency = v;
                    }
                }
                "years_experience" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.years_experience = v;
                    }
                }
                "verification" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.verification = v;
                    }
                }
                "verified_by" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.verified_by = v;
                    }
                }
                "last_used_at" => {
                    if let Ok(v) = serde_json::from_value(value) {
                        self.last_used_at = v;
                    }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for EmployeeSkill {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "EmployeeSkill"
    }
}

impl backbone_core::PersistentEntity for EmployeeSkill {
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

impl backbone_orm::EntityRepoMeta for EmployeeSkill {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("employee_id".to_string(), "uuid".to_string());
        m.insert("skill_id".to_string(), "uuid".to_string());
        m.insert("proficiency".to_string(), "proficiency_level".to_string());
        m.insert("verification".to_string(), "skill_verification".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &[]
    }
}

/// Builder for EmployeeSkill entity
///
/// Provides a fluent API for constructing EmployeeSkill instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct EmployeeSkillBuilder {
    employee_id: Option<Uuid>,
    skill_id: Option<Uuid>,
    proficiency: Option<ProficiencyLevel>,
    years_experience: Option<Decimal>,
    verification: Option<SkillVerification>,
    verified_by: Option<Uuid>,
    last_used_at: Option<NaiveDate>,
}

impl EmployeeSkillBuilder {
    /// Set the employee_id field (required)
    pub fn employee_id(mut self, value: Uuid) -> Self {
        self.employee_id = Some(value);
        self
    }

    /// Set the skill_id field (required)
    pub fn skill_id(mut self, value: Uuid) -> Self {
        self.skill_id = Some(value);
        self
    }

    /// Set the proficiency field (default: `ProficiencyLevel::default()`)
    pub fn proficiency(mut self, value: ProficiencyLevel) -> Self {
        self.proficiency = Some(value);
        self
    }

    /// Set the years_experience field (optional)
    pub fn years_experience(mut self, value: Decimal) -> Self {
        self.years_experience = Some(value);
        self
    }

    /// Set the verification field (default: `SkillVerification::default()`)
    pub fn verification(mut self, value: SkillVerification) -> Self {
        self.verification = Some(value);
        self
    }

    /// Set the verified_by field (optional)
    pub fn verified_by(mut self, value: Uuid) -> Self {
        self.verified_by = Some(value);
        self
    }

    /// Set the last_used_at field (optional)
    pub fn last_used_at(mut self, value: NaiveDate) -> Self {
        self.last_used_at = Some(value);
        self
    }

    /// Build the EmployeeSkill entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<EmployeeSkill, String> {
        let employee_id = self
            .employee_id
            .ok_or_else(|| "employee_id is required".to_string())?;
        let skill_id = self
            .skill_id
            .ok_or_else(|| "skill_id is required".to_string())?;

        Ok(EmployeeSkill {
            id: Uuid::new_v4(),
            employee_id,
            skill_id,
            proficiency: self.proficiency.unwrap_or(ProficiencyLevel::default()),
            years_experience: self.years_experience,
            verification: self.verification.unwrap_or(SkillVerification::default()),
            verified_by: self.verified_by,
            last_used_at: self.last_used_at,
            metadata: AuditMetadata::default(),
        })
    }
}
