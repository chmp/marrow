//! # `marrow` - a minimalistic Arrow implementation
//!
//! `marrow` allows building and viewing arrow arrays of different implementations using a unified
//! interface. The motivation behind `marrow` is to allow libraries to target multiple different
//! arrow versions simultaneously.
//!
//! ## Conversions
//!
//! `marrow` offers conversions between its types and the types of different arrow versions. See the
//! [features](#features) section how to enable support.
//!
//! From `marrow` to `arrow`:
//!
//! - `TryFrom<marrow::array::Array> for arrow::array::ArrayRef`
//! - `TryFrom<&marrow::datatypes::DataType> for arrow::datatypes::DataType`
//! - `TryFrom<&marrow::datatypes::Field> for arrow::datatypes::Field`
//! - `TryFrom<marrow::datatypes::TimeUnit> for arrow::datatypes::TimeUnit`
//! - `TryFrom<marrow::datatypes::UnionMode> for arrow::datatypes::UnionMode`
//!
//! From `arrow` to `marrow`:
//!
//! - `TryFrom<&dyn arrow::array::Array> for marrow::view::View<'_>`
//! - `TryFrom<&arrow::datatypes::DataType> for marrow::datatypes::DataType`
//! - `TryFrom<&arrow::datatypes::Field> for marrow::datatypes::Field`
//! - `TryFrom<arrow::datatypes::TimeUnit> for marrow::datatypes::TimeUnit`
//! - `TryFrom<arrow::datatypes::UnionMode> for marrow::datatypes::UnionMode`
//!
//! ## Features
//!
//! Supported features:
//!
//! - `serde`: enable Serde serialization / deserialization for schema types
//!   ([Field][crate::datatypes::Field], [DataType][crate::datatypes::DataType], ...). The format
//!   will match the one of the `arrow` crate
//! - `arrow-{major}`: enable conversions between `arrow={major}` and `marrow`
//! - `arrow2-0-{minor}`: enable conversions between `arrow2=0.{minor}` and `marrow`
//!
//! This crate supports conversions from and to different version of `arrow` or `arrow2`. These
//! conversions can be enabled by selecting the relevant features. Any combination of features can
//! be selected, e.g., both `arrow-53` and `arrow-52` can be used at the same time.
//!
//! Available features:
//!
//! | Feature       | Arrow Version |
//! |---------------|---------------|
// arrow-version:insert: //! | `arrow-{version}`    | `arrow={version}`    |
//! | `arrow-53`    | `arrow=53`    |
//! | `arrow-52`    | `arrow=52`    |
//! | `arrow-51`    | `arrow=51`    |
//! | `arrow-50`    | `arrow=50`    |
//! | `arrow-49`    | `arrow=49`    |
//! | `arrow-48`    | `arrow=48`    |
//! | `arrow-47`    | `arrow=47`    |
//! | `arrow-46`    | `arrow=46`    |
//! | `arrow-45`    | `arrow=45`    |
//! | `arrow-44`    | `arrow=44`    |
//! | `arrow-43`    | `arrow=43`    |
//! | `arrow-42`    | `arrow=42`    |
//! | `arrow-41`    | `arrow=41`    |
//! | `arrow-40`    | `arrow=40`    |
//! | `arrow-39`    | `arrow=39`    |
//! | `arrow-38`    | `arrow=38`    |
//! | `arrow-37`    | `arrow=37`    |
//! | `arrow2-0-17` | `arrow2=0.17` |
//! | `arrow2-0-16` | `arrow2=0.16` |

pub mod array;
pub mod datatypes;
pub mod error;
pub mod meta;
pub mod view;

mod impl_arrow;
mod impl_arrow2;
