//! Arrays with borrowed data
//!
//! The views correspond 1:1 to the corresponding arrays.
use half::f16;

use crate::datatypes::{FieldMeta, TimeUnit};

/// An array with borrowed data
///
/// See [`Array`][crate::array::Array]
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
///
/// The `i`-th element is stored at bit `offset + i`.
#[derive(Debug, Clone, Copy)]
pub struct BitsWithOffset<'a> {
    /// The offset of the bits
    pub offset: usize,
    /// The data of the bitmap
    pub data: &'a [u8],
}

/// See [`NullArray`][crate::array::NullArray]
#[derive(Clone, Debug)]
pub struct NullView {
    /// See [`NullArray::len`][crate::array::NullArray::len]
    pub len: usize,
}

/// See [`BooleanArray`][crate::array::BooleanArray]
#[derive(Clone, Debug)]
pub struct BooleanView<'a> {
    /// See [`BooleanArray::len`][crate::array::BooleanArray::len]
    pub len: usize,
    /// See [`BooleanArray::validity`][crate::array::BooleanArray::validity]
    pub validity: Option<BitsWithOffset<'a>>,
    /// See [`BooleanArray::values`][crate::array::BooleanArray::values]
    pub values: BitsWithOffset<'a>,
}

/// See [`PrimitiveArray`][crate::array::PrimitiveArray]
#[derive(Clone, Debug)]
pub struct PrimitiveView<'a, T> {
    /// See [`PrimitiveArray::validity`][crate::array::PrimitiveArray::validity]
    pub validity: Option<BitsWithOffset<'a>>,
    /// See [`PrimitiveArray::values`][crate::array::PrimitiveArray::values]
    pub values: &'a [T],
}

/// See [`TimeArray`][crate::array::TimeArray]
#[derive(Debug, Clone)]
pub struct TimeView<'a, T> {
    /// See [`TimeArray::unit`][crate::array::TimeArray::unit]
    pub unit: TimeUnit,
    /// See [`TimeArray::validity`][crate::array::TimeArray::validity]
    pub validity: Option<BitsWithOffset<'a>>,
    /// See [`TimeArray::values`][crate::array::TimeArray::values]
    pub values: &'a [T],
}

/// See [`TimestampArray`][crate::array::TimestampArray]
#[derive(Debug, Clone)]
pub struct TimestampView<'a> {
    /// See [`TimestampArray::unit`][crate::array::TimestampArray::unit]
    pub unit: TimeUnit,
    /// See [`TimestampArray::timezone`][crate::array::TimestampArray::timezone]
    pub timezone: Option<String>,
    /// See [`TimestampArray::validity`][crate::array::TimestampArray::validity]
    pub validity: Option<BitsWithOffset<'a>>,
    /// See [`TimestampArray::values`][crate::array::TimestampArray::values]
    pub values: &'a [i64],
}

/// See [`StructArray`][crate::array::StructArray]
#[derive(Clone, Debug)]
pub struct StructView<'a> {
    /// See [`StructArray::len`][crate::array::StructArray::len]
    pub len: usize,
    /// See [`StructArray::validity`][crate::array::StructArray::validity]
    pub validity: Option<BitsWithOffset<'a>>,
    /// See [`StructArray::fields`][crate::array::StructArray::fields]
    pub fields: Vec<(View<'a>, FieldMeta)>,
}

/// See [`ListArray`][crate::array::ListArray]
#[derive(Clone, Debug)]
pub struct ListView<'a, O> {
    /// See [`ListArray::validity`][crate::array::ListArray::validity]
    pub validity: Option<BitsWithOffset<'a>>,
    /// See [`ListArray::offsets`][crate::array::ListArray::offsets]
    pub offsets: &'a [O],
    /// See [`ListArray::meta`][crate::array::ListArray::meta]
    pub meta: FieldMeta,
    /// See [`ListArray::elements`][crate::array::ListArray::elements]
    pub elements: Box<View<'a>>,
}

/// See [`FixedSizeListArray`][crate::array::FixedSizeListArray]
#[derive(Clone, Debug)]
pub struct FixedSizeListView<'a> {
    /// See [`FixedSizeListArray::len`][crate::array::FixedSizeListArray::len]
    pub len: usize,
    /// See [`FixedSizeListArray::n`][crate::array::FixedSizeListArray::n]
    pub n: i32,
    /// See [`FixedSizeListArray::validity`][crate::array::BytesArray::validity]
    pub validity: Option<BitsWithOffset<'a>>,
    /// See [`FixedSizeListArray::meta`][crate::array::FixedSizeListArray::meta]
    pub meta: FieldMeta,
    /// See [`FixedSizeListArray::elements`][crate::array::FixedSizeListArray::elements]
    pub elements: Box<View<'a>>,
}

/// See [`BytesArray`][crate::array::BytesArray]
#[derive(Clone, Debug)]
pub struct BytesView<'a, O> {
    /// See [`BytesArray::validity`][crate::array::BytesArray::validity]
    pub validity: Option<BitsWithOffset<'a>>,
    /// See [`BytesArray::offsets`][crate::array::BytesArray::offsets]
    pub offsets: &'a [O],
    /// See [`BytesArray::data`][crate::array::BytesArray::data]
    pub data: &'a [u8],
}

/// See [`FixedSizeBinaryArray`][crate::array::FixedSizeBinaryArray]
#[derive(Clone, Debug)]
pub struct FixedSizeBinaryView<'a> {
    /// See [`FixedSizeBinaryArray::n`][crate::array::FixedSizeBinaryArray::n]
    pub n: i32,
    /// See [`FixedSizeBinaryArray::validity`][crate::array::FixedSizeBinaryArray::validity]
    pub validity: Option<BitsWithOffset<'a>>,
    /// See [`FixedSizeBinaryArray::data`][crate::array::FixedSizeBinaryArray::data]    
    pub data: &'a [u8],
}

/// See [`DecimalArray`][crate::array::DecimalArray]
#[derive(Clone, Debug)]
pub struct DecimalView<'a, T> {
    /// See [`DecimalArray::precision`][crate::array::DecimalArray::precision]
    pub precision: u8,
    /// See [`DecimalArray::scale`][crate::array::DecimalArray::scale]
    pub scale: i8,
    /// See [`DecimalArray::validity`][crate::array::DecimalArray::validity]
    pub validity: Option<BitsWithOffset<'a>>,
    /// See [`DecimalArray::values`][crate::array::DecimalArray::values]
    pub values: &'a [T],
}

/// See [`DictionaryArray`][crate::array::DictionaryArray]
#[derive(Clone, Debug)]
pub struct DictionaryView<'a> {
    /// See [`DictionaryArray::indices`][crate::array::DictionaryArray::indices]
    pub indices: Box<View<'a>>,
    /// See [`DictionaryArray::values`][crate::array::DictionaryArray::values]
    pub values: Box<View<'a>>,
}

/// See [`DenseUnionArray`][crate::array::DenseUnionArray]
#[derive(Clone, Debug)]
pub struct DenseUnionView<'a> {
    /// See [`DenseUnionArray::types`][crate::array::DenseUnionArray::types]
    pub types: &'a [i8],
    /// See [`DenseUnionArray::offsets`][crate::array::DenseUnionArray::offsets]
    pub offsets: &'a [i32],
    /// See [`DenseUnionArray::fields`][crate::array::DenseUnionArray::fields]
    pub fields: Vec<(i8, View<'a>, FieldMeta)>,
}
