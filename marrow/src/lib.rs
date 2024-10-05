//! A common arrow abstraction to simplify using different arrow implementations
//!
//! # Features
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
