mod error;
mod impls;
mod typeinfo;

#[cfg(test)]
mod tests;

/// Derive [TypeInfo] for a given type
///
/// Currently structs and enums with any type of lifetime parameters are supported.
pub use marrow_convert_derive::TypeInfo;

pub use error::{Error, Result};
pub use typeinfo::{Context, Options, TypeInfo, get_data_type, get_field};
