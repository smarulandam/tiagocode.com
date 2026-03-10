use serde::de::Error as _;
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

pub fn deserialize_number_from_string<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum NumberLike {
        String(String),
        I64(i64),
        U64(u64),
        F64(f64),
    }

    let input = NumberLike::deserialize(deserializer)?;
    let raw = match input {
        NumberLike::String(value) => value,
        NumberLike::I64(value) => value.to_string(),
        NumberLike::U64(value) => value.to_string(),
        NumberLike::F64(value) => value.to_string(),
    };

    raw.parse::<T>()
        .map_err(|error| D::Error::custom(format!("invalid numeric value `{raw}`: {error}")))
}

pub fn deserialize_bool_from_anything<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum BoolLike {
        Bool(bool),
        I64(i64),
        U64(u64),
        String(String),
    }

    let value = BoolLike::deserialize(deserializer)?;
    match value {
        BoolLike::Bool(value) => Ok(value),
        BoolLike::I64(value) => Ok(value != 0),
        BoolLike::U64(value) => Ok(value != 0),
        BoolLike::String(value) => {
            let normalized = value.trim().to_ascii_lowercase();
            match normalized.as_str() {
                "true" | "t" | "yes" | "y" | "on" | "1" => Ok(true),
                "false" | "f" | "no" | "n" | "off" | "0" => Ok(false),
                _ => Err(D::Error::custom(format!("invalid boolean value `{value}`"))),
            }
        }
    }
}
