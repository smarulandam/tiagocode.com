use std::fmt::Display;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Forbidden: {reason}")]
    Forbidden { reason: String },

    #[error("Not found: {resource} ({id})")]
    NotFound { resource: &'static str, id: String },

    #[error("Validation failed for {field}: {reason}")]
    Validation { field: &'static str, reason: String },

    #[error("External system error ({system}): {message}")]
    External {
        system: &'static str,
        message: String,
    },

    #[error("Decode error for {target}: {message}")]
    Decode {
        target: &'static str,
        message: String,
    },

    #[error("Encode error for {target}: {message}")]
    Encode {
        target: &'static str,
        message: String,
    },

    #[error("Unexpected error: {message}")]
    Unexpected { message: String },
}

impl AppError {
    pub fn external(system: &'static str, error: impl Display) -> Self {
        Self::External {
            system,
            message: error.to_string(),
        }
    }

    pub fn decode(target: &'static str, error: impl Display) -> Self {
        Self::Decode {
            target,
            message: error.to_string(),
        }
    }

    pub fn encode(target: &'static str, error: impl Display) -> Self {
        Self::Encode {
            target,
            message: error.to_string(),
        }
    }

    pub fn validation(field: &'static str, reason: impl Into<String>) -> Self {
        Self::Validation {
            field,
            reason: reason.into(),
        }
    }

    pub fn unexpected(error: impl Display) -> Self {
        Self::Unexpected {
            message: error.to_string(),
        }
    }
}

pub type Result<T> = core::result::Result<T, AppError>;
