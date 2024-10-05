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

#[derive(Clone, Debug)]
pub struct PrimitiveArray<T> {
    /// The validity of the elements as a bitmap
    pub validity: Option<Vec<u8>>,
    pub values: Vec<T>,
}

#[derive(Debug, Clone)]
pub struct TimeArray<T> {
    pub unit: TimeUnit,
    /// The validity of the elements as a bitmap
    pub validity: Option<Vec<u8>>,
    pub values: Vec<T>,
}

#[derive(Debug, Clone)]

pub struct TimestampArray {
    pub unit: TimeUnit,
    pub timezone: Option<String>,
    /// The validity of the elements as a bitmap
    pub validity: Option<Vec<u8>>,
    pub values: Vec<i64>,
}

#[derive(Clone, Debug)]
pub struct StructArray {
    pub len: usize,
    /// The validity of the elements as a bitmap
    pub validity: Option<Vec<u8>>,
    pub fields: Vec<(Array, FieldMeta)>,
}

#[derive(Clone, Debug)]
pub struct ListArray<O> {
    /// The validity of the elements as a bitmap
    pub validity: Option<Vec<u8>>,
    pub offsets: Vec<O>,
    pub meta: FieldMeta,
    pub element: Box<Array>,
}

/// An array comprised of lists of fixed size
#[derive(Clone, Debug)]
pub struct FixedSizeListArray {
    /// The number of elements in this array, each a list with `n` children
    pub len: usize,
    /// The number of children per element
    pub n: i32,
    /// The validity of the elements as a bitmap
    pub validity: Option<Vec<u8>>,
    pub meta: FieldMeta,
    pub element: Box<Array>,
}

#[derive(Clone, Debug)]
pub struct BytesArray<O> {
    /// The validity of the elements as a bitmap
    pub validity: Option<Vec<u8>>,
    pub offsets: Vec<O>,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct FixedSizeBinaryArray {
    pub n: i32,
    /// The validity of the elements as a bitmap
    pub validity: Option<Vec<u8>>,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct DecimalArray<T> {
    pub precision: u8,
    pub scale: i8,
    /// The validity of the elements as a bitmap
    pub validity: Option<Vec<u8>>,
    pub values: Vec<T>,
}

#[derive(Clone, Debug)]
pub struct DictionaryArray {
    pub indices: Box<Array>,
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
