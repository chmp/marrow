#![deny(rustdoc::broken_intra_doc_links)]
mod error;
mod internal;

#[cfg(test)]
mod tests;

pub use error::{Error, Result};

/// Traits to derive schema information from a type
pub mod types {
    pub use crate::internal::type_info::{
        Context, DefaultArrayType, Options, get_data_type, get_field,
    };

    /// Derive [DefaultArrayType] for a given Rust type
    ///
    /// Currently structs and enums without type generic are supported.
    pub use marrow_convert_derive::DefaultArrayType;
}

/// Traits to allow constructing arrays from Rust objects
pub mod builder {
    pub use crate::internal::builder::list::{LargeListBuilder, ListBuilder};
    pub use crate::internal::builder::primitive::{
        BooleanBuilder, Float16Builder, Float32Builder, Float64Builder, Int8Builder, Int16Builder,
        Int32Builder, Int64Builder, NullBuilder, UInt8Builder, UInt16Builder, UInt32Builder,
        UInt64Builder,
    };
    pub use crate::internal::builder::{ArrayBuilder, ArrayPush, DefaultArrayBuilder};

    /// Collect builders to simplify implementing custom builders for compound types (structs and
    /// enums)
    pub mod compound {
        pub use crate::internal::builder::{
            r#struct::StructBuilder,
            union::{DenseTypes, DenseUnionBuilder, SparseUnionBuilder},
        };
    }

    /// Derive [ArrayPush] for a given type
    pub use marrow_convert_derive::ArrayPush;

    /// Derive [DefaultArrayBuilder] for a given type
    pub use marrow_convert_derive::DefaultArrayBuilder;
}
