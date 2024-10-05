//! Views into array data
//!
//! As each view corresponds 1:1 to the corresponding array, the docs refer to their docs.
use half::f16;

use crate::{datatypes::TimeUnit, meta::FieldMeta};

/// A view into the data of an array
///
/// The data is owned by an external array. See [`Array`][crate::array::Array] for docs on the
/// different variants.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum View<'a> {
    /// See [`Array::Null`][crate::array::Array::Null]
    Null(NullView),
    /// See [`Array::Boolean`][crate::array::Array::Boolean]
    Boolean(BooleanView<'a>),
    /// See [`Array::Int8`][crate::array::Array::Int8]
    Int8(PrimitiveView<'a, i8>),
    /// See [`Array::Int16`][crate::array::Array::Int16]
    Int16(PrimitiveView<'a, i16>),
    /// See [`Array::Int32`][crate::array::Array::Int32]
    Int32(PrimitiveView<'a, i32>),
    /// See [`Array::Int64`][crate::array::Array::Int64]
    Int64(PrimitiveView<'a, i64>),
    /// See [`Array::UInt8`][crate::array::Array::UInt8]
    UInt8(PrimitiveView<'a, u8>),
    /// See [`Array::UInt16`][crate::array::Array::UInt16]
    UInt16(PrimitiveView<'a, u16>),
    /// See [`Array::UInt32`][crate::array::Array::UInt32]
    UInt32(PrimitiveView<'a, u32>),
    /// See [`Array::UInt64`][crate::array::Array::UInt64]
    UInt64(PrimitiveView<'a, u64>),
    /// See [`Array::Float16`][crate::array::Array::Float16]
    Float16(PrimitiveView<'a, f16>),
    /// See [`Array::Float32`][crate::array::Array::Float32]
    Float32(PrimitiveView<'a, f32>),
    /// See [`Array::Float64`][crate::array::Array::Float64]
    Float64(PrimitiveView<'a, f64>),
    /// See [`Array::Date32`][crate::array::Array::Date32]
    Date32(PrimitiveView<'a, i32>),
    /// See [`Array::Date64`][crate::array::Array::Date64]
    Date64(PrimitiveView<'a, i64>),
    /// See [`Array::Time32`][crate::array::Array::Time32]
    Time32(TimeView<'a, i32>),
    /// See [`Array::Time64`][crate::array::Array::Time64]
    Time64(TimeView<'a, i64>),
    /// See [`Array::Timestamp`][crate::array::Array::Timestamp]
    Timestamp(TimestampView<'a>),
    /// See [`Array::Duration`][crate::array::Array::Duration]
    Duration(TimeView<'a, i64>),
    /// See [`Array::Utf8`][crate::array::Array::Utf8]
    Utf8(BytesView<'a, i32>),
    /// See [`Array::LargeUtf8`][crate::array::Array::LargeUtf8]
    LargeUtf8(BytesView<'a, i64>),
    /// See [`Array::Binary`][crate::array::Array::Binary]
    Binary(BytesView<'a, i32>),
    /// See [`Array::LargeBinary`][crate::array::Array::LargeBinary]
    LargeBinary(BytesView<'a, i64>),
    /// See [`Array::FixedSizeBinary`][crate::array::Array::FixedSizeBinary]
    FixedSizeBinary(FixedSizeBinaryView<'a>),
    /// See [`Array::Decimal128`][crate::array::Array::Decimal128]
    Decimal128(DecimalView<'a, i128>),
    /// See [`Array::Struct`][crate::array::Array::Struct]
    Struct(StructView<'a>),
    /// See [`Array::List`][crate::array::Array::List]
    List(ListView<'a, i32>),
    /// See [`Array::LargeList`][crate::array::Array::LargeList]
    LargeList(ListView<'a, i64>),
    /// See [`Array::FixedSizeList`][crate::array::Array::FixedSizeList]
    FixedSizeList(FixedSizeListView<'a>),
    /// See [`Array::Dictionary`][crate::array::Array::Dictionary]
    Dictionary(DictionaryView<'a>),
    /// See [`Array::Map`][crate::array::Array::Map]
    Map(ListView<'a, i32>),
    /// See [`Array::DenseUnion`][crate::array::Array::DenseUnion]
    DenseUnion(DenseUnionView<'a>),
}

/// A bitmap with an optional offset
#[derive(Debug, Clone, Copy)]
pub struct BitsWithOffset<'a> {
    /// The offset of the bits
    ///
    /// The `i`-th element is stored at bit `offset + i`.
    pub offset: usize,
    /// The data of the bitmap
    pub data: &'a [u8],
}

/// See [`NullArray`][crate::array::NullArray]
#[derive(Clone, Debug)]
pub struct NullView {
    pub len: usize,
}

/// See [`BooleanArray`][crate::array::BooleanArray]
#[derive(Clone, Debug)]
pub struct BooleanView<'a> {
    pub len: usize,
    pub validity: Option<BitsWithOffset<'a>>,
    pub values: BitsWithOffset<'a>,
}

/// See [`PrimitiveArray`][crate::array::PrimitiveArray]
#[derive(Clone, Debug)]
pub struct PrimitiveView<'a, T> {
    pub validity: Option<BitsWithOffset<'a>>,
    pub values: &'a [T],
}

/// See [`TimeArray`][crate::array::TimeArray]
#[derive(Debug, Clone)]
pub struct TimeView<'a, T> {
    pub unit: TimeUnit,
    pub validity: Option<BitsWithOffset<'a>>,
    pub values: &'a [T],
}

/// See [`TimestampArray`][crate::array::TimestampArray]
#[derive(Debug, Clone)]
pub struct TimestampView<'a> {
    pub unit: TimeUnit,
    pub timezone: Option<String>,
    pub validity: Option<BitsWithOffset<'a>>,
    pub values: &'a [i64],
}

/// See [`StructArray`][crate::array::StructArray]
#[derive(Clone, Debug)]
pub struct StructView<'a> {
    pub len: usize,
    pub validity: Option<BitsWithOffset<'a>>,
    pub fields: Vec<(View<'a>, FieldMeta)>,
}

/// See [`ListArray`][crate::array::ListArray]
#[derive(Clone, Debug)]
pub struct ListView<'a, O> {
    pub validity: Option<BitsWithOffset<'a>>,
    pub offsets: &'a [O],
    pub meta: FieldMeta,
    pub element: Box<View<'a>>,
}

/// See [`FixedSizeListArray`][crate::array::FixedSizeListArray]
#[derive(Clone, Debug)]
pub struct FixedSizeListView<'a> {
    pub len: usize,
    pub n: i32,
    pub validity: Option<BitsWithOffset<'a>>,
    pub meta: FieldMeta,
    pub element: Box<View<'a>>,
}

/// See [`BytesArray`][crate::array::BytesArray]
#[derive(Clone, Debug)]
pub struct BytesView<'a, O> {
    pub validity: Option<BitsWithOffset<'a>>,
    pub offsets: &'a [O],
    pub data: &'a [u8],
}

/// See [`FixedSizeBinaryArray`][crate::array::FixedSizeBinaryArray]
#[derive(Clone, Debug)]
pub struct FixedSizeBinaryView<'a> {
    pub n: i32,
    pub validity: Option<BitsWithOffset<'a>>,
    pub data: &'a [u8],
}

/// See [`DecimalArray`][crate::array::DecimalArray]
#[derive(Clone, Debug)]
pub struct DecimalView<'a, T> {
    pub precision: u8,
    pub scale: i8,
    pub validity: Option<BitsWithOffset<'a>>,
    pub values: &'a [T],
}

/// See [`DictionaryArray`][crate::array::DictionaryArray]
#[derive(Clone, Debug)]
pub struct DictionaryView<'a> {
    pub indices: Box<View<'a>>,
    pub values: Box<View<'a>>,
}

/// See [`DenseUnionArray`][crate::array::DenseUnionArray]
#[derive(Clone, Debug)]
pub struct DenseUnionView<'a> {
    pub types: &'a [i8],
    pub offsets: &'a [i32],
    pub fields: Vec<(i8, View<'a>, FieldMeta)>,
}
