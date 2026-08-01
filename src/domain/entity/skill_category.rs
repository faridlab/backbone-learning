use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "skill_category", rename_all = "snake_case")]
pub enum SkillCategory {
    Technical,
    Soft,
    Tool,
    Language,
    Domain,
    Certification,
}

impl std::fmt::Display for SkillCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Technical => write!(f, "technical"),
            Self::Soft => write!(f, "soft"),
            Self::Tool => write!(f, "tool"),
            Self::Language => write!(f, "language"),
            Self::Domain => write!(f, "domain"),
            Self::Certification => write!(f, "certification"),
        }
    }
}

impl FromStr for SkillCategory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "technical" => Ok(Self::Technical),
            "soft" => Ok(Self::Soft),
            "tool" => Ok(Self::Tool),
            "language" => Ok(Self::Language),
            "domain" => Ok(Self::Domain),
            "certification" => Ok(Self::Certification),
            _ => Err(format!("Unknown SkillCategory variant: {}", s)),
        }
    }
}

impl Default for SkillCategory {
    fn default() -> Self {
        Self::Technical
    }
}
