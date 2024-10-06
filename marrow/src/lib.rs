//! # `marrow` - minimalist Arrow interop
//!
//! `marrow` allows building and viewing arrow arrays of different implementations using a unified
//! interface. The motivation behind `marrow` is to allow libraries to target multiple different
//! arrow versions simultaneously.
//!
//! The main type are
//!
//! - [`Array`][crate::array::Array]: an array with owned data
//! - [`View`][crate::view::View]: an array with borrowed data
//! - [`Field`][crate::datatypes::Field]: field metadata
//! - [`DataType`][crate::datatypes::DataType]: field data types
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
//! For `arrow2` the corresponding conversions are implemented as well.
//!
#![cfg_attr(
// arrow-version: replace:     feature = "arrow-{version}",
    feature = "arrow-53",
    doc = r#"
For example to access the data in an arrow array:

```rust
# use marrow::_impl::arrow as arrow;
# fn main() -> marrow::error::Result<()> {
use arrow::array::Int32Array;
use marrow::view::View;

// build the arrow array
let arrow_array = Int32Array::from(vec![Some(1), Some(2), Some(3)]);

// construct the view into this array
let marrow_view = View::try_from(&arrow_array as &dyn arrow::array::Array)?;

// access the underlying data
let View::Int32(marrow_view) = marrow_view else { panic!() };
assert_eq!(marrow_view.values, &[1, 2, 3]);
#     Ok(())
# }
```

Or to build an array:

```rust
# use marrow::_impl::arrow as arrow;
# fn main() -> marrow::error::Result<()> {
use arrow::array::Array as _;
use marrow::array::{Array, PrimitiveArray};

// build the array
let marrow_array = Array::Int32(PrimitiveArray {
    validity: Some(vec![0b_101]),
    values: vec![4, 0, 6],
});

// convert it to an arrow array
let arrow_array_ref = arrow::array::ArrayRef::try_from(marrow_array)?;
assert_eq!(arrow_array_ref.is_null(0), false);
assert_eq!(arrow_array_ref.is_null(1), true);
assert_eq!(arrow_array_ref.is_null(2), false);
#     Ok(())
# }
```
"#
)]
//!
//! ## Features
//!
//! Supported features:
//!
//! - `serde`: enable Serde serialization / deserialization for schema types
//!   ([Field][crate::datatypes::Field], [DataType][crate::datatypes::DataType], ...). The format
//!   will match the one of the `arrow` crate
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

pub mod _impl;
#[deny(missing_docs)]
pub mod array;
#[deny(missing_docs)]
pub mod datatypes;
#[deny(missing_docs)]
pub mod error;
#[deny(missing_docs)]
pub mod meta;
#[deny(missing_docs)]
pub mod view;

mod impl_arrow;
mod impl_arrow2;
