use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "course_format", rename_all = "snake_case")]
pub enum CourseFormat {
    Online,
    Onsite,
    Blended,
    SelfPaced,
}

impl std::fmt::Display for CourseFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Online => write!(f, "online"),
            Self::Onsite => write!(f, "onsite"),
            Self::Blended => write!(f, "blended"),
            Self::SelfPaced => write!(f, "self_paced"),
        }
    }
}

impl FromStr for CourseFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "online" => Ok(Self::Online),
            "onsite" => Ok(Self::Onsite),
            "blended" => Ok(Self::Blended),
            "self_paced" => Ok(Self::SelfPaced),
            _ => Err(format!("Unknown CourseFormat variant: {}", s)),
        }
    }
}

impl Default for CourseFormat {
    fn default() -> Self {
        Self::Online
    }
}
