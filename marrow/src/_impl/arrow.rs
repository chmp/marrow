//! a module that mirrors the arrow package for the most recent release

// arrow-version: replace: use arrow_array_{version} as _arrow_array;
use arrow_array_53 as _arrow_array;

// arrow-version: replace: use arrow_schema_{version} as _arrow_schema;
use arrow_schema_53 as _arrow_schema;

pub mod array {
    pub use super::_arrow_array::array::{
        make_array, Array, ArrayRef, ArrowPrimitiveType, BooleanArray, DictionaryArray,
        FixedSizeBinaryArray, FixedSizeListArray, GenericBinaryArray, GenericListArray,
        GenericStringArray, LargeStringArray, MapArray, NullArray, OffsetSizeTrait, PrimitiveArray,
        StringArray, StructArray, UnionArray,
    };
    pub use super::_arrow_array::RecordBatch;
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
