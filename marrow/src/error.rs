//! Error handling in `marrow`
//!

/// A result type that defaults to `marrow::Error`
pub type Result<T, E = MarrowError> = std::result::Result<T, E>;

/// Errors that may occur in `marrow`
pub struct MarrowError(Box<ErrorImpl>);

/// The kind of error to simplify matching against known error conditions
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ErrorKind {
    /// Errors encountered during string parsing
    ParseError,
    /// An error raised in the Arrow implementation used
    ArrowError,
    /// Unsupported operations or arrow features
    Unsupported,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        macro_rules! write_variant_name {
            ($($variant:ident),* $(,)?) => {
                match self {
                    $(
                        Self::$variant => write!(f, stringify!($variant)),
                    )*
                }
            };
        }

        write_variant_name!(ParseError, ArrowError, Unsupported)
    }
}

impl MarrowError {
    /// The kind of the error
    pub fn kind(&self) -> ErrorKind {
        self.0.kind
    }

    /// The message attached to the error
    pub fn message(&self) -> &str {
        &self.0.message
    }

    /// The backtrace captured when the error was constructed
    pub fn backtrace(&self) -> &Backtrace {
        &self.0.backtrace
    }
}

impl std::error::Error for MarrowError {
    fn description(&self) -> &str {
        self.message()
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.0.cause {
            Some(error) => Some(error.as_ref()),
            None => None,
        }
    }
}

struct ErrorImpl {
    kind: ErrorKind,
    message: String,
    backtrace: Backtrace,
    cause: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Debug for MarrowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.0.message)
    }
}

impl std::fmt::Display for MarrowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.0.message)
    }
}

macro_rules! fail {
    ($kind:expr, $($msg:tt)*) => {
        return Err($crate::error::error_with_kind_and_message($kind, format!($($msg)*)))
    };
}

use std::backtrace::Backtrace;

pub(crate) use fail;

pub(crate) fn error_with_kind_and_message(kind: ErrorKind, message: String) -> MarrowError {
    MarrowError(Box::new(ErrorImpl {
        kind,
        message,
        backtrace: Backtrace::capture(),
        cause: None,
    }))
}

pub(crate) fn error_with_kind_message_cause(
    kind: ErrorKind,
    message: String,
    cause: impl std::error::Error + Send + Sync + 'static,
) -> MarrowError {
    MarrowError(Box::new(ErrorImpl {
        kind,
        message,
        backtrace: Backtrace::capture(),
        cause: Some(Box::new(cause)),
    }))
}

impl From<std::num::TryFromIntError> for MarrowError {
    fn from(err: std::num::TryFromIntError) -> MarrowError {
        error_with_kind_message_cause(
            ErrorKind::Unsupported,
            format!("TryFromIntError: {err}"),
            err,
        )
    }
}
