use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "competency_category", rename_all = "snake_case")]
pub enum CompetencyCategory {
    Technical,
    Leadership,
    SoftSkill,
    Safety,
    Compliance,
}

impl std::fmt::Display for CompetencyCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Technical => write!(f, "technical"),
            Self::Leadership => write!(f, "leadership"),
            Self::SoftSkill => write!(f, "soft_skill"),
            Self::Safety => write!(f, "safety"),
            Self::Compliance => write!(f, "compliance"),
        }
    }
}

impl FromStr for CompetencyCategory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "technical" => Ok(Self::Technical),
            "leadership" => Ok(Self::Leadership),
            "soft_skill" => Ok(Self::SoftSkill),
            "safety" => Ok(Self::Safety),
            "compliance" => Ok(Self::Compliance),
            _ => Err(format!("Unknown CompetencyCategory variant: {}", s)),
        }
    }
}

impl Default for CompetencyCategory {
    fn default() -> Self {
        Self::Technical
    }
}
