use serde::{Deserialize, Serialize};
use std::any::type_name;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use uuid::Uuid;

use crate::application::domain::core::AppError;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Identifier(Uuid);

impl Identifier {
    pub fn new(uuid: Uuid) -> Result<Self, AppError> {
        if !uuid.is_nil() && !uuid.is_max() {
            return Ok(Self(uuid));
        }

        let uuid = uuid.to_string();
        Err(AppError::validation(
            type_name::<Self>(),
            format!("invalid value `{}`", uuid),
        ))
    }
}

impl TryFrom<&str> for Identifier {
    type Error = AppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let uuid = Uuid::try_from(value).map_err(|_| {
            AppError::validation(type_name::<Self>(), format!("invalid value `{}`", value))
        })?;
        Self::new(uuid)
    }
}

impl TryFrom<String> for Identifier {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl Deref for Identifier {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("413b8ba1-2bc4-4fda-8455-0c0dea528ca0")]
    fn creation_succeeds_when_input_is_valid(#[case] input: &str) {
        let identifier = Identifier::try_from(input).unwrap();
        assert_eq!(identifier.to_string(), input.to_string());
    }

    #[rstest]
    #[case("invalid-uuid")]
    #[case("00000000-0000-0000-0000-000000000000")]
    #[case("ffffffff-ffff-ffff-ffff-ffffffffffff")]
    #[should_panic]
    fn creation_fails_when_input_is_invalid(#[case] input: &str) {
        Identifier::try_from(input).unwrap();
    }

    #[test]
    fn display_succeeds_when_formatting_to_string() {
        let uuid_str = "413b8ba1-2bc4-4fda-8455-0c0dea528ca0";
        let identifier = Identifier::try_from(uuid_str).unwrap();
        assert_eq!(identifier.to_string(), uuid_str.to_string())
    }

    #[test]
    fn comparison_succeeds_when_variants_are_the_same() {
        let uuid_str = "413b8ba1-2bc4-4fda-8455-0c0dea528ca0";
        let identifier = Identifier::try_from(uuid_str).unwrap();
        let other_identifier = Identifier::try_from(uuid_str).unwrap();

        assert_eq!(identifier, other_identifier);
    }
}
