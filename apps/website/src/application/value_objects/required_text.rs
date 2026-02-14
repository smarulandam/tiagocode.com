use serde::{Deserialize, Serialize};
use std::any::type_name;
use std::ops::Deref;

use crate::application::domain::core::AppError;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequiredText(String);

impl RequiredText {
    pub fn new(value: String) -> Result<Self, AppError> {
        match value.trim().is_empty() {
            false => Ok(Self(value)),
            true => Err(AppError::validation(
                type_name::<Self>(),
                format!("invalid value `{}`", value),
            )),
        }
    }
}

impl TryFrom<String> for RequiredText {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for RequiredText {
    type Error = AppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string())
    }
}

impl Deref for RequiredText {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("This is an example text")]
    #[case("This is another example text")]
    #[case("...")]
    fn creation_succeeds_when_input_is_valid_text(#[case] input: &str) {
        assert_eq!(*RequiredText::try_from(input).unwrap(), input)
    }

    #[rstest]
    #[case("")]
    #[case("       ")]
    #[case("\r\n")]
    #[case("\t")]
    #[should_panic]
    fn creation_fails_when_input_contains_only_scape_sequences(#[case] input: &str) {
        RequiredText::try_from(input).unwrap();
    }

    #[test]
    fn display_succeeds_when_formatting_to_string() {
        let text = RequiredText::try_from("This is an example text").unwrap();
        assert_eq!(text.to_string(), "This is an example text");
    }

    #[test]
    fn comparison_succeeds_when_values_are_the_same() {
        let first_text = RequiredText::try_from("This is an example text").unwrap();
        let second_text = RequiredText::try_from("This is an example text").unwrap();

        assert_eq!(first_text, second_text);
    }
}
