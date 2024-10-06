//! Owned arrays
use half::f16;

use crate::{datatypes::TimeUnit, meta::FieldMeta};

/// An owned array
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Array {
    /// An array without data
    Null(NullArray),
    /// `bool` array
    Boolean(BooleanArray),
    /// `i8` array
    Int8(PrimitiveArray<i8>),
    /// `i16` array
    Int16(PrimitiveArray<i16>),
    /// `i32` array
    Int32(PrimitiveArray<i32>),
    /// `i64` array
    Int64(PrimitiveArray<i64>),
    /// `u8` array
    UInt8(PrimitiveArray<u8>),
    /// `u16` array
    UInt16(PrimitiveArray<u16>),
    /// `u32` array
    UInt32(PrimitiveArray<u32>),
    /// `u64` array
    UInt64(PrimitiveArray<u64>),
    /// `f16` array
    Float16(PrimitiveArray<f16>),
    /// `f32` array
    Float32(PrimitiveArray<f32>),
    /// `f64` array
    Float64(PrimitiveArray<f64>),
    /// An `i32` array of dates
    Date32(PrimitiveArray<i32>),
    /// An `i64` array of dates
    Date64(PrimitiveArray<i64>),
    /// An `i32` array of times
    Time32(TimeArray<i32>),
    /// An `i64` array of times
    Time64(TimeArray<i64>),
    /// An `i64` array of timestamps
    Timestamp(TimestampArray),
    /// An `i64` array of durations
    Duration(TimeArray<i64>),
    /// A `[u8]` array with `i32` offsets of strings
    Utf8(BytesArray<i32>),
    /// A `[u8]` array with `i64` offsets of strings
    LargeUtf8(BytesArray<i64>),
    /// A `[u8]` array with `i32` offsets
    Binary(BytesArray<i32>),
    /// A `[u8]` array with `i64` offsets
    LargeBinary(BytesArray<i64>),
    /// A `[u8; N]` array with `i32` offsets
    FixedSizeBinary(FixedSizeBinaryArray),
    /// An `i128` array of decimals
    Decimal128(DecimalArray<i128>),
    /// An array of structs
    Struct(StructArray),
    /// An array of lists with `i32` offsets
    List(ListArray<i32>),
    /// An array of lists with `i64` offsets
    LargeList(ListArray<i64>),
    /// An array of fixed sized list with `i32` offsets
    FixedSizeList(FixedSizeListArray),
    /// An array of dictionaries
    Dictionary(DictionaryArray),
    /// An array of maps
    Map(ListArray<i32>),
    /// An array of unions
    DenseUnion(DenseUnionArray),
}

/// An array without data
#[derive(Clone, Debug)]
pub struct NullArray {
    /// The len of the array
    pub len: usize,
}

/// A `bool` array
#[derive(Clone, Debug)]
pub struct BooleanArray {
    // Note: len is required to know how many bits of values are used
    /// The len of the array
    pub len: usize,
    /// The validity of the elements as a bitmap
    pub validity: Option<Vec<u8>>,
    /// The values as a bitmap
    pub values: Vec<u8>,
}

/// An array of primitive values
#[derive(Clone, Debug)]
pub struct PrimitiveArray<T> {
    /// The validity of the elements as a bitmap
    pub validity: Option<Vec<u8>>,
    /// The values of the array
    pub values: Vec<T>,
}

/// An array time values (e.g., `"12:53"`)
#[derive(Debug, Clone)]
pub struct TimeArray<T> {
    /// The time unit of the values
    pub unit: TimeUnit,
    /// The validity of the elements as a bitmap
    pub validity: Option<Vec<u8>>,
    /// The values of the array stored as the offsets from midnight
    pub values: Vec<T>,
}

/// An array of timestamps with an optional timezone
#[derive(Debug, Clone)]

pub struct TimestampArray {
    /// The time unit of the values
    pub unit: TimeUnit,
    /// THe optional timezone
    pub timezone: Option<String>,
    /// The validity of the elements as a bitmap
    pub validity: Option<Vec<u8>>,
    /// The values as UTC timestamps
    pub values: Vec<i64>,
}

/// An array of structs
#[derive(Clone, Debug)]
pub struct StructArray {
    /// The number of elements in the array
    pub len: usize,
    /// The validity of the elements as a bitmap
    pub validity: Option<Vec<u8>>,
    /// The fields with their metadata
    pub fields: Vec<(Array, FieldMeta)>,
}

/// An array of lists
/// 
/// The value element `i` is given by the pseudo code `elements[offsets[i]..[offsets[i+1]]`
#[derive(Clone, Debug)]
pub struct ListArray<O> {
    /// The validity of the elements as a bitmap
    pub validity: Option<Vec<u8>>,
    /// The offsets of the elements
    pub offsets: Vec<O>,
    /// The metadata of the elements array
    pub meta: FieldMeta,
    /// The values stored in the array
    pub elements: Box<Array>,
}

/// An array of lists of fixed size
#[derive(Clone, Debug)]
pub struct FixedSizeListArray {
    /// The number of elements in this array, each a list with `n` children
    pub len: usize,
    /// The number of children per element
    pub n: i32,
    /// The validity of the elements as a bitmap
    pub validity: Option<Vec<u8>>,
    /// The metadata of the elements array
    pub meta: FieldMeta,
    /// The values stored in the array
    pub elements: Box<Array>,
}

/// An array of bytes with varying sizes
///
/// The value of element `i` can be access by the pseudo code `data[offsets[i]..offsets[i + 1]]`
#[derive(Clone, Debug)]
pub struct BytesArray<O> {
    /// The validity of the elements as a bitmap
    pub validity: Option<Vec<u8>>,
    /// The offsets into the data array the first element is `0`
    pub offsets: Vec<O>,
    /// The underlying data with all elements concatenated
    pub data: Vec<u8>,
}

/// An array of byte vectors with fixed length
#[derive(Clone, Debug)]
pub struct FixedSizeBinaryArray {
    /// The number of bytes per element
    pub n: i32,
    /// The validity of the elements as a bitmap
    pub validity: Option<Vec<u8>>,
    /// The data with each element concatenated
    pub data: Vec<u8>,
}

/// An array of fixed point values
///
/// The value of element `i` can be computed by the pseudo code: `values[i] * (10 ** -scale)`
#[derive(Clone, Debug)]
pub struct DecimalArray<T> {
    /// The precision, i.e., the number of digits
    pub precision: u8,
    /// The scale, i.e., the position of smallest value that can be represented
    pub scale: i8,
    /// The validity of the elements as a bitmap
    pub validity: Option<Vec<u8>>,
    /// The underlying values
    pub values: Vec<T>,
}

/// An array that deduplicates elements
///
/// For element `i`, the value can be looked up by the pseudo code `values[indices[i]]`
#[derive(Clone, Debug)]
pub struct DictionaryArray {
    /// The indices into the values array for each element
    pub indices: Box<Array>,
    /// The possible values of elements
    pub values: Box<Array>,
}

/// A union of different data types
///
/// This corresponds roughly to Rust's enums. Each element has a type, which indicates the
/// underlying array to use. For fast lookups the offsets into the underlying arrays are stored as
/// well. For element `ì`, the value can be looked up by the pseudo code
/// `fields[types[i]].1[offsets[i]]`.
#[derive(Clone, Debug)]
pub struct DenseUnionArray {
    /// The type of each element
    pub types: Vec<i8>,
    /// The offset into the underlying arrays
    pub offsets: Vec<i32>,
    /// The arrays with their metadata
    pub fields: Vec<(i8, Array, FieldMeta)>,
}
