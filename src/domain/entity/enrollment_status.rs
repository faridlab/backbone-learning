use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "enrollment_status", rename_all = "snake_case")]
pub enum EnrollmentStatus {
    Enrolled,
    InProgress,
    Completed,
    Withdrawn,
    Failed,
}

impl std::fmt::Display for EnrollmentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Enrolled => write!(f, "enrolled"),
            Self::InProgress => write!(f, "in_progress"),
            Self::Completed => write!(f, "completed"),
            Self::Withdrawn => write!(f, "withdrawn"),
            Self::Failed => write!(f, "failed"),
        }
    }
}

impl FromStr for EnrollmentStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "enrolled" => Ok(Self::Enrolled),
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "withdrawn" => Ok(Self::Withdrawn),
            "failed" => Ok(Self::Failed),
            _ => Err(format!("Unknown EnrollmentStatus variant: {}", s)),
        }
    }
}

impl Default for EnrollmentStatus {
    fn default() -> Self {
        Self::Enrolled
    }
}
