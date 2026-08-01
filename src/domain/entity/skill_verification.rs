use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "skill_verification", rename_all = "snake_case")]
pub enum SkillVerification {
    SelfReported,
    Assessed,
    Certified,
}

impl std::fmt::Display for SkillVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SelfReported => write!(f, "self_reported"),
            Self::Assessed => write!(f, "assessed"),
            Self::Certified => write!(f, "certified"),
        }
    }
}

impl FromStr for SkillVerification {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "self_reported" => Ok(Self::SelfReported),
            "assessed" => Ok(Self::Assessed),
            "certified" => Ok(Self::Certified),
            _ => Err(format!("Unknown SkillVerification variant: {}", s)),
        }
    }
}

impl Default for SkillVerification {
    fn default() -> Self {
        Self::SelfReported
    }
}
