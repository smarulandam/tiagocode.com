use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ModerationStatus {
    Published,
    Unpublished,
}

impl From<bool> for ModerationStatus {
    fn from(value: bool) -> Self {
        match value {
            true => ModerationStatus::Published,
            _ => ModerationStatus::Unpublished,
        }
    }
}

impl Display for ModerationStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ModerationStatus::Published => write!(f, "Published"),
            ModerationStatus::Unpublished => write!(f, "Unpublished"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation_succeeds_when_input_is_boolean() {
        assert_eq!(ModerationStatus::from(true), ModerationStatus::Published);
        assert_eq!(ModerationStatus::from(false), ModerationStatus::Unpublished);
    }

    #[test]
    fn displays_succeeds_when_formatting_to_string() {
        assert_eq!(ModerationStatus::Published.to_string(), "Published");
        assert_eq!(ModerationStatus::Unpublished.to_string(), "Unpublished")
    }

    #[test]
    fn comparison_succeeds_when_variants_are_the_same() {
        assert_eq!(ModerationStatus::Published, ModerationStatus::Published);
        assert_eq!(ModerationStatus::Unpublished, ModerationStatus::Unpublished);
    }
}
