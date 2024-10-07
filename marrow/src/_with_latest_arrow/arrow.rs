//! a module that mirrors the arrow package for the most recent release
#![allow(unused)]

// arrow-version: replace: use arrow_array_{version} as _arrow_array;
use arrow_array_53 as _arrow_array;

// arrow-version: replace: use arrow_schema_{version} as _arrow_schema;
use arrow_schema_53 as _arrow_schema;

pub mod array {
    pub use super::_arrow_array::array::{
        make_array, Array, ArrayRef, ArrowPrimitiveType, DictionaryArray, GenericBinaryArray,
        GenericListArray, GenericStringArray, OffsetSizeTrait, PrimitiveArray,
    };
    pub use super::_arrow_array::RecordBatch;

    // specialized arrays
    pub use super::_arrow_array::array::{
        BinaryArray, BooleanArray, FixedSizeBinaryArray, FixedSizeListArray, Float16Array,
        Float32Array, Float64Array, Int16Array, Int32Array, Int64Array, Int8Array,
        LargeBinaryArray, LargeStringArray, MapArray, NullArray, StringArray, StructArray,
        UInt16Array, UInt32Array, UInt64Array, UInt8Array, UnionArray,
    };

    // specialized builders
    pub use super::_arrow_array::builder::{
        FixedSizeListBuilder, Int32Builder, LargeListBuilder, ListBuilder,
    };
}
pub mod datatypes {
    pub use super::_arrow_array::types::{
        ArrowDictionaryKeyType, ArrowPrimitiveType, Date32Type, Date64Type, Decimal128Type,
        DurationMicrosecondType, DurationMillisecondType, DurationNanosecondType,
        DurationSecondType, Float16Type, Float32Type, Float64Type, Int16Type, Int32Type, Int64Type,
        Int8Type, Time32MillisecondType, Time32SecondType, Time64MicrosecondType,
        Time64NanosecondType, TimestampMicrosecondType, TimestampMillisecondType,
        TimestampNanosecondType, TimestampSecondType, UInt16Type, UInt32Type, UInt64Type,
        UInt8Type,
    };
    pub use super::_arrow_schema::{DataType, Field, FieldRef, Schema, TimeUnit, UnionMode};
}
pub mod error {
    pub use super::_arrow_schema::ArrowError;
}
