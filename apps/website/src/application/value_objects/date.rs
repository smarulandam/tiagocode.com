use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::any::type_name;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

use crate::application::domain::core::AppError;

/// A value object that represents a date.
///
/// This type wraps a UTC [`DateTime`] and provides functionality to create,
/// parse and display dates in a consistent format. All dates are internally
/// stored in UTC to ensure consistency across different time zones.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Date(DateTime<Utc>);

impl Date {
    pub fn new(date_time: DateTime<Utc>) -> Self {
        Self(date_time)
    }

    pub fn to_string_with_format(&self, format: &str) -> String {
        self.0.format(format).to_string()
    }
}

impl TryFrom<&str> for Date {
    type Error = AppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        DateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S%:z")
            .map_err(|_| {
                AppError::validation(type_name::<Self>(), format!("invalid value `{}`", value))
            })
            .map(|datetime| Self::new(datetime.to_utc()))
    }
}

impl TryFrom<String> for Date {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl Deref for Date {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.naive_utc().to_string())
    }
}

impl PartialEq<Date> for Date {
    fn eq(&self, other: &Date) -> bool {
        self.0.naive_utc().to_string() == other.0.naive_utc().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("2024-12-15T14:03:56+00:00")]
    #[case("1997-12-15T14:03:56+00:00")]
    #[case("2028-12-15T14:03:56+00:00")]
    fn creation_succeeds_when_input_is_iso8601(#[case] input: &str) {
        Date::try_from(input).unwrap();
    }

    #[rstest]
    #[case("2024/03/01")]
    #[case("1st March 2024")]
    #[should_panic]
    fn creation_fails_when_input_is_any_different_format(#[case] input: &str) {
        Date::try_from(input).unwrap();
    }

    #[test]
    fn display_succeeds_when_formatting_to_string() {
        let date = Date::try_from("2024-12-15T14:03:56+00:00").unwrap();
        assert_eq!(date.to_string(), "2024-12-15 14:03:56");
    }

    #[rstest]
    #[case("2024-12-15T14:03:56+00:00", "%B %d, %Y", "December 15, 2024")]
    fn display_succeeds_when_using_custom_date_formats(
        #[case] input: &str,
        #[case] format: &str,
        #[case] expected: &str,
    ) {
        let date = Date::try_from(input).unwrap();
        assert_eq!(date.to_string_with_format(format), expected);
    }

    #[test]
    fn comparison_succeeds_when_values_are_the_same() {
        let date = Date::try_from("2024-12-15T14:03:56+00:00").unwrap();
        let other_date = Date::try_from("2024-12-15T14:03:56+00:00").unwrap();
        assert_eq!(date, other_date);
    }
}
