//! Helpers for tests

/// Helper to view an as the given variant
macro_rules! view_as {
    ($variant:path, $array_ref:expr) => {
        match crate::view::View::try_from(&*$array_ref) {
            Ok($variant(view)) => Ok(view),
            Ok(view) => Err(crate::error::error_with_kind_and_message(
                crate::error::ErrorKind::Unsupported,
                format!(
                    "Unexpected view: expected {expected}, got {actual:?}",
                    expected = stringify!($variant),
                    actual = view,
                ),
            )),
            Err(err) => Err(err),
        }
    };
}

pub(crate) use view_as;

#[derive(Debug)]
pub(crate) struct PanicOnErrorError;

impl<E: std::error::Error> From<E> for PanicOnErrorError {
    fn from(err: E) -> Self {
        panic!("{err:?}")
    }
}

#[allow(unused)]
pub(crate) type PanicOnError<T, E = PanicOnErrorError> = std::result::Result<T, E>;
