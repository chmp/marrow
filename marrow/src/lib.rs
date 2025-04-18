//! # `marrow` - minimalist Arrow interop
//!
//! `marrow` allows building and viewing arrow arrays of different implementations using a unified
//! interface. The motivation behind `marrow` is to allow libraries to target multiple different
//! arrow versions simultaneously.
//!
//! Supported arrow implementations:
//!
//! - [`arrow`](https://docs.rs/arrow/)
//! - [`arrow2`](https://docs.rs/arrow2)
//!
//! The main types are
//!
//! - [`Array`][crate::array::Array]: an array with owned data
//! - [`View`][crate::view::View]: an array with borrowed data
//! - [`Field`][crate::datatypes::Field]: the data type and metadata of a field
//! - [`DataType`][crate::datatypes::DataType]: data types of arrays
//!
//! ## Conversions
//!
//! `marrow` offers conversions between its types and the types of different arrow versions. See the
//! [features](#features) section how to enable support for a specific version. The following
//! conversion are implemented.
//!
//! From `marrow` to `arrow`:
//!
//! - `TryFrom<`[`marrow::array::Array`][crate::array::Array]`> for arrow::array::ArrayRef`
//! - `TryFrom<&`[`marrow::datatypes::Field`][crate::datatypes::Field]`> for arrow::datatypes::Field`
//! - `TryFrom<&`[`marrow::datatypes::DataType`][crate::datatypes::DataType]`> for arrow::datatypes::DataType`
//! - `TryFrom<`[`marrow::datatypes::TimeUnit`][crate::datatypes::TimeUnit]`> for arrow::datatypes::TimeUnit`
//! - `TryFrom<`[`marrow::datatypes::UnionMode`][crate::datatypes::UnionMode]`> for arrow::datatypes::UnionMode`
//!
//! From `arrow` to `marrow`:
//!
//! - `TryFrom<&dyn arrow::array::Array> for `[`marrow::view::View<'_>`][crate::view::View]
//! - `TryFrom<&arrow::datatypes::Field> for `[`marrow::datatypes::Field`][crate::datatypes::Field]
//! - `TryFrom<&arrow::datatypes::DataType> for `[`marrow::datatypes::DataType`][crate::datatypes::DataType]
//! - `TryFrom<arrow::datatypes::TimeUnit> for `[`marrow::datatypes::TimeUnit`][crate::datatypes::TimeUnit]
//! - `TryFrom<arrow::datatypes::UnionMode> for `[`marrow::datatypes::UnionMode`][crate::datatypes::UnionMode]
//!
//! For `arrow2` the corresponding conversions are implemented.
//!
//! For example to access the data in an arrow array:
//!
//! ```rust
//! # fn main() -> marrow::error::Result<()> { marrow::_with_arrow! {
//! use arrow::array::Int32Array;
//! use marrow::view::View;
//!
//! // build the arrow array
//! let arrow_array = Int32Array::from(vec![Some(1), Some(2), Some(3)]);
//!
//! // construct a view of this array
//! let marrow_view = View::try_from(&arrow_array as &dyn arrow::array::Array)?;
//!
//! // access the underlying data
//! let View::Int32(marrow_view) = marrow_view else { panic!() };
//! assert_eq!(marrow_view.values, &[1, 2, 3]);
//! # } }
//! ```
//!
//! Or to build an array:
//!
//! ```rust
//! # fn main() -> marrow::error::Result<()> { marrow::_with_arrow! {
//! use arrow::array::Array as _;
//! use marrow::array::{Array, PrimitiveArray};
//!
//! // build the array
//! let marrow_array = Array::Int32(PrimitiveArray {
//!     validity: Some(marrow::bit_vec![true, false, true]),
//!     values: vec![4, 0, 6],
//! });
//!
//! // convert it to an arrow array
//! let arrow_array_ref = arrow::array::ArrayRef::try_from(marrow_array)?;
//! assert_eq!(arrow_array_ref.is_null(0), false);
//! assert_eq!(arrow_array_ref.is_null(1), true);
//! assert_eq!(arrow_array_ref.is_null(2), false);
//! # } }
//! ```
//!
//! ## Features
//!
//! Supported features:
//!
//! - `serde`: enable Serde serialization / deserialization for schema types
//!   ([Field][crate::datatypes::Field], [DataType][crate::datatypes::DataType], ...). The format
//!   will match the `arrow` crate
//! - `arrow-{version}`: enable conversions between `marrow` and `arrow={version}`
//! - `arrow2-{version}`: enable conversions between `marrow` and `arrow2={version}`
//!
//! This crate supports conversions from and to different version of `arrow` or `arrow2`. These
//! conversions can be enabled by selecting the relevant features. Any combination of features can
//! be selected, e.g., both `arrow-53` and `arrow-52` can be used at the same time.
//!
//! Supported arrow versions:
//!
//! | Feature       | Arrow Version |
//! |---------------|---------------|
// arrow-version:insert: //! | `arrow-{version}`    | `arrow={version}`    |
//! | `arrow-55`    | `arrow=55`    |
//! | `arrow-54`    | `arrow=54`    |
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
//!
//! Note, `arrow2=0.18` is not supported as the source code was not tagged on GitHub.
//!
#[deny(missing_docs)]
pub mod array;
#[deny(missing_docs)]
pub mod datatypes;
#[deny(missing_docs)]
pub mod error;
#[deny(missing_docs)]
pub mod types;
#[deny(missing_docs)]
pub mod view;

#[deny(missing_docs)]
pub mod bits;

mod impl_arrow;
mod impl_arrow2;

#[doc(hidden)]
pub mod r#impl;
