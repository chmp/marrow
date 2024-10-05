use std::borrow::Cow;

use crate::{
    array::{Array, PrimitiveArray},
    datatypes::{DataType, Field, TimeUnit, UnionMode},
    error::{MarrowError, ErrorKind, Result, error_with_kind_message_cause, fail},
    meta::{meta_from_field, FieldMeta},
    view::{
        BitsWithOffset, BooleanView, BytesView, DecimalView, DenseUnionView, DictionaryView,
        FixedSizeBinaryView, FixedSizeListView, ListView, NullView, PrimitiveView, StructView,
        TimeView, TimestampView, View,
    },
};

impl From<arrow2::error::Error> for MarrowError {
    fn from(err: arrow2::error::Error) -> MarrowError {
        error_with_kind_message_cause(ErrorKind::ArrowError, format!("arrow2::Error: {err}"), err)
    }
}

/// Conversion from `arrow2` data types
impl TryFrom<&arrow2::datatypes::DataType> for DataType {
    type Error = MarrowError;

    fn try_from(value: &arrow2::datatypes::DataType) -> Result<DataType> {
        use {arrow2::datatypes::DataType as AT, DataType as T, Field as F, arrow2::datatypes::IntegerType as I};
        match value {
            AT::Null => Ok(T::Null),
            AT::Boolean => Ok(T::Boolean),
            AT::Int8 => Ok(T::Int8),
            AT::Int16 => Ok(T::Int16),
            AT::Int32 => Ok(T::Int32),
            AT::Int64 => Ok(T::Int64),
            AT::UInt8 => Ok(T::UInt8),
            AT::UInt16 => Ok(T::UInt16),
            AT::UInt32 => Ok(T::UInt32),
            AT::UInt64 => Ok(T::UInt64),
            AT::Float16 => Ok(T::Float16),
            AT::Float32 => Ok(T::Float32),
            AT::Float64 => Ok(T::Float64),
            AT::Date32 => Ok(T::Date32),
            AT::Date64 => Ok(T::Date64),
            AT::Time32(unit) => Ok(T::Time32((*unit).into())),
            AT::Time64(unit) => Ok(T::Time64((*unit).into())),
            AT::Duration(unit) => Ok(T::Duration((*unit).into())),
            AT::Timestamp(unit, tz) => Ok(T::Timestamp((*unit).into(), tz.clone())),
            AT::Decimal(precision, scale) => {
                if *precision > u8::MAX as usize || *scale > i8::MAX as usize {
                    fail!(ErrorKind::Unsupported, "cannot represent precision / scale of the decimal");
                }
                Ok(T::Decimal128(*precision as u8, *scale as i8))
            }
            AT::Utf8 => Ok(T::Utf8),
            AT::LargeUtf8 => Ok(T::LargeUtf8),
            AT::Binary => Ok(T::Binary),
            AT::LargeBinary => Ok(T::LargeBinary),
            AT::FixedSizeBinary(n) => Ok(T::FixedSizeBinary(i32::try_from(*n)?)),
            AT::List(entry) => Ok(T::List(Box::new(entry.as_ref().try_into()?))),
            AT::LargeList(entry) => Ok(T::LargeList(Box::new(entry.as_ref().try_into()?))),
            AT::FixedSizeList(entry, n) => Ok(T::FixedSizeList(
                Box::new(entry.as_ref().try_into()?),
                i32::try_from(*n)?,
            )),
            AT::Map(field, sorted) => Ok(T::Map(Box::new(field.as_ref().try_into()?), *sorted)),
            AT::Struct(fields) => {
                let mut res_fields = Vec::new();
                for field in fields {
                    res_fields.push(Field::try_from(field)?);
                }
                Ok(T::Struct(res_fields))
            }
            AT::Dictionary(key, value, sorted) => {
                let key = match key {
                    I::Int8 => T::Int8,
                    I::Int16 => T::Int16,
                    I::Int32 => T::Int32,
                    I::Int64 => T::Int64,
                    I::UInt8 => T::UInt8,
                    I::UInt16 => T::UInt16,
                    I::UInt32 => T::UInt32,
                    I::UInt64 => T::UInt64,
                };
                Ok(T::Dictionary(
                    Box::new(key),
                    Box::new(value.as_ref().try_into()?),
                    *sorted,
                ))
            }
            AT::Union(in_fields, in_type_ids, mode) => {
                let in_type_ids = match in_type_ids {
                    Some(in_type_ids) => in_type_ids.clone(),
                    None => {
                        let mut type_ids = Vec::new();
                        for id in 0..in_fields.len() {
                            type_ids.push(id.try_into()?);
                        }
                        type_ids
                    }
                };

                let mut fields = Vec::new();
                for (type_id, field) in in_type_ids.iter().zip(in_fields) {
                    fields.push(((*type_id).try_into()?, F::try_from(field)?));
                }
                Ok(T::Union(fields, (*mode).into()))
            }
            dt => fail!(ErrorKind::Unsupported, "Cannot convert data type {dt:?} to internal data type"),
        }
    }
}

/// Conversion from `arrow2` fields
impl TryFrom<&arrow2::datatypes::Field> for Field {
    type Error = MarrowError;

    fn try_from(field: &arrow2::datatypes::Field) -> Result<Self> {
        Ok(Field {
            name: field.name.to_owned(),
            data_type: DataType::try_from(&field.data_type)?,
            nullable: field.is_nullable,
            metadata: field.metadata.clone().into_iter().collect(),
        })
    }
}


/// Conversion to `arrow2` data types
impl TryFrom<&DataType> for arrow2::datatypes::DataType {
    type Error = MarrowError;

    fn try_from(value: &DataType) -> std::result::Result<Self, Self::Error> {
        use {arrow2::datatypes::DataType as AT, arrow2::datatypes::Field as AF, DataType as T, arrow2::datatypes::IntegerType as I};
        match value {
            T::Null => Ok(AT::Null),
            T::Boolean => Ok(AT::Boolean),
            T::Int8 => Ok(AT::Int8),
            T::Int16 => Ok(AT::Int16),
            T::Int32 => Ok(AT::Int32),
            T::Int64 => Ok(AT::Int64),
            T::UInt8 => Ok(AT::UInt8),
            T::UInt16 => Ok(AT::UInt16),
            T::UInt32 => Ok(AT::UInt32),
            T::UInt64 => Ok(AT::UInt64),
            T::Float16 => Ok(AT::Float16),
            T::Float32 => Ok(AT::Float32),
            T::Float64 => Ok(AT::Float64),
            T::Date32 => Ok(AT::Date32),
            T::Date64 => Ok(AT::Date64),
            T::Duration(unit) => Ok(AT::Duration((*unit).into())),
            T::Time32(unit) => Ok(AT::Time32((*unit).into())),
            T::Time64(unit) => Ok(AT::Time64((*unit).into())),
            T::Timestamp(unit, tz) => Ok(AT::Timestamp((*unit).into(), tz.clone())),
            T::Decimal128(precision, scale) => {
                if *scale < 0 {
                    fail!(ErrorKind::Unsupported, "arrow2 does not support decimals with negative scale");
                }
                Ok(AT::Decimal((*precision).into(), (*scale).try_into()?))
            }
            T::Binary => Ok(AT::Binary),
            T::LargeBinary => Ok(AT::LargeBinary),
            T::FixedSizeBinary(n) => Ok(AT::FixedSizeBinary((*n).try_into()?)),
            T::Utf8 => Ok(AT::Utf8),
            T::LargeUtf8 => Ok(AT::LargeUtf8),
            T::Dictionary(key, value, sorted) => match key.as_ref() {
                T::Int8 => Ok(AT::Dictionary(
                    I::Int8,
                    AT::try_from(value.as_ref())?.into(),
                    *sorted,
                )),
                T::Int16 => Ok(AT::Dictionary(
                    I::Int16,
                    AT::try_from(value.as_ref())?.into(),
                    *sorted,
                )),
                T::Int32 => Ok(AT::Dictionary(
                    I::Int32,
                    AT::try_from(value.as_ref())?.into(),
                    *sorted,
                )),
                T::Int64 => Ok(AT::Dictionary(
                    I::Int64,
                    AT::try_from(value.as_ref())?.into(),
                    *sorted,
                )),
                T::UInt8 => Ok(AT::Dictionary(
                    I::UInt8,
                    AT::try_from(value.as_ref())?.into(),
                    *sorted,
                )),
                T::UInt16 => Ok(AT::Dictionary(
                    I::UInt16,
                    AT::try_from(value.as_ref())?.into(),
                    *sorted,
                )),
                T::UInt32 => Ok(AT::Dictionary(
                    I::UInt32,
                    AT::try_from(value.as_ref())?.into(),
                    *sorted,
                )),
                T::UInt64 => Ok(AT::Dictionary(
                    I::UInt64,
                    AT::try_from(value.as_ref())?.into(),
                    *sorted,
                )),
                dt => fail!(
                    ErrorKind::Unsupported,
                    "unsupported dictionary key type {dt:?}",
                ),
            },
            T::List(field) => Ok(AT::List(AF::try_from(field.as_ref())?.into())),
            T::LargeList(field) => Ok(AT::LargeList(AF::try_from(field.as_ref())?.into())),
            T::FixedSizeList(field, n) => Ok(AT::FixedSizeList(
                AF::try_from(field.as_ref())?.into(),
                (*n).try_into()?,
            )),
            T::Map(field, sorted) => Ok(AT::Map(AF::try_from(field.as_ref())?.into(), *sorted)),
            T::Struct(in_fields) => {
                let mut fields = Vec::new();
                for field in in_fields {
                    fields.push(AF::try_from(field)?);
                }
                Ok(AT::Struct(fields))
            }
            T::Union(in_fields, mode) => {
                let mut fields = Vec::new();
                let mut type_ids = Vec::new();

                for (type_id, field) in in_fields {
                    fields.push(AF::try_from(field)?);
                    type_ids.push((*type_id).into());
                }
                Ok(AT::Union(fields, Some(type_ids), (*mode).into()))
            }
        }
    }
}

/// Conversion to `arrow2` fields
impl TryFrom<&Field> for arrow2::datatypes::Field {
    type Error = MarrowError;

    fn try_from(value: &Field) -> Result<Self> {
        Ok(arrow2::datatypes::Field {
            name: value.name.to_owned(),
            data_type: arrow2::datatypes::DataType::try_from(&value.data_type)?,
            is_nullable: value.nullable,
            metadata: value.metadata.clone().into_iter().collect(),
        })
    }
}

/// Conversion to `arrow2` time units
impl From<TimeUnit> for arrow2::datatypes::TimeUnit {
    fn from(value: TimeUnit) -> arrow2::datatypes::TimeUnit {
        match value {
            TimeUnit::Second => arrow2::datatypes::TimeUnit::Second,
            TimeUnit::Millisecond => arrow2::datatypes::TimeUnit::Millisecond,
            TimeUnit::Microsecond => arrow2::datatypes::TimeUnit::Microsecond,
            TimeUnit::Nanosecond => arrow2::datatypes::TimeUnit::Nanosecond,
        }
    }
}

/// Conversion from `arrow2` time units
impl From<arrow2::datatypes::TimeUnit> for TimeUnit {
    fn from(value: arrow2::datatypes::TimeUnit) -> TimeUnit {
        match value {
            arrow2::datatypes::TimeUnit::Second => TimeUnit::Second,
            arrow2::datatypes::TimeUnit::Millisecond => TimeUnit::Millisecond,
            arrow2::datatypes::TimeUnit::Microsecond => TimeUnit::Microsecond,
            arrow2::datatypes::TimeUnit::Nanosecond => TimeUnit::Nanosecond,
        }
    }
}

/// Conversion from `arrow2` union modes
impl From<arrow2::datatypes::UnionMode> for UnionMode {
    fn from(value: arrow2::datatypes::UnionMode) -> Self {
        match value {
            arrow2::datatypes::UnionMode::Dense => UnionMode::Dense,
            arrow2::datatypes::UnionMode::Sparse => UnionMode::Sparse,
        }
    }
}

/// Conversion to `arrow2` union modes
impl From<UnionMode> for arrow2::datatypes::UnionMode {
    fn from(value: UnionMode) -> Self {
        match value {
            UnionMode::Dense => arrow2::datatypes::UnionMode::Dense,
            UnionMode::Sparse => arrow2::datatypes::UnionMode::Sparse,
        }
    }
}

/// Conversion to `arrow2` arrays
impl TryFrom<Array> for Box<dyn arrow2::array::Array> {
    type Error = MarrowError;

    fn try_from(value: Array) -> Result<Self> {
        use {arrow2::datatypes::DataType as AT, arrow2::datatypes::IntegerType as AI, Array as A};
        match value {
            A::Null(arr) => Ok(Box::new(arrow2::array::NullArray::new(AT::Null, arr.len))),
            A::Boolean(arr) => Ok(Box::new(arrow2::array::BooleanArray::try_new(
                AT::Boolean,
                arrow2::bitmap::Bitmap::from_u8_vec(arr.values, arr.len),
                arr.validity
                    .map(|v| arrow2::bitmap::Bitmap::from_u8_vec(v, arr.len)),
            )?)),
            A::Int8(arr) => build_primitive_array(AT::Int8, arr.values, arr.validity),
            A::Int16(arr) => build_primitive_array(AT::Int16, arr.values, arr.validity),
            A::Int32(arr) => build_primitive_array(AT::Int32, arr.values, arr.validity),
            A::Int64(arr) => build_primitive_array(AT::Int64, arr.values, arr.validity),
            A::UInt8(arr) => build_primitive_array(AT::UInt8, arr.values, arr.validity),
            A::UInt16(arr) => build_primitive_array(AT::UInt16, arr.values, arr.validity),
            A::UInt32(arr) => build_primitive_array(AT::UInt32, arr.values, arr.validity),
            A::UInt64(arr) => build_primitive_array(AT::UInt64, arr.values, arr.validity),
            A::Float16(arr) => build_primitive_array(
                AT::Float16,
                arr.values
                    .into_iter()
                    .map(|v| arrow2::types::f16::from_bits(v.to_bits()))
                    .collect(),
                arr.validity,
            ),
            A::Float32(arr) => build_primitive_array(AT::Float32, arr.values, arr.validity),
            A::Float64(arr) => build_primitive_array(AT::Float64, arr.values, arr.validity),
            A::Date32(arr) => build_primitive_array(AT::Date32, arr.values, arr.validity),
            A::Date64(arr) => build_primitive_array(AT::Date64, arr.values, arr.validity),
            A::Duration(arr) => {
                build_primitive_array(AT::Duration(arr.unit.into()), arr.values, arr.validity)
            }
            A::Time32(arr) => {
                build_primitive_array(AT::Time32(arr.unit.into()), arr.values, arr.validity)
            }
            A::Time64(arr) => {
                build_primitive_array(AT::Time64(arr.unit.into()), arr.values, arr.validity)
            }
            A::Timestamp(arr) => build_primitive_array(
                AT::Timestamp(arr.unit.into(), arr.timezone),
                arr.values,
                arr.validity,
            ),
            A::Decimal128(arr) => build_primitive_array(
                AT::Decimal(arr.precision as usize, usize::try_from(arr.scale)?),
                arr.values,
                arr.validity,
            ),
            A::Utf8(arr) => build_utf8_array(AT::Utf8, arr.offsets, arr.data, arr.validity),
            A::LargeUtf8(arr) => {
                build_utf8_array(AT::LargeUtf8, arr.offsets, arr.data, arr.validity)
            }
            A::Binary(arr) => build_binary_array(AT::Binary, arr.offsets, arr.data, arr.validity),
            A::LargeBinary(arr) => {
                build_binary_array(AT::LargeBinary, arr.offsets, arr.data, arr.validity)
            }
            A::Dictionary(arr) => match *arr.indices {
                A::Int8(indices) => build_dictionary_array(AI::Int8, indices, *arr.values),
                A::Int16(indices) => build_dictionary_array(AI::Int16, indices, *arr.values),
                A::Int32(indices) => build_dictionary_array(AI::Int32, indices, *arr.values),
                A::Int64(indices) => build_dictionary_array(AI::Int64, indices, *arr.values),
                A::UInt8(indices) => build_dictionary_array(AI::UInt8, indices, *arr.values),
                A::UInt16(indices) => build_dictionary_array(AI::UInt16, indices, *arr.values),
                A::UInt32(indices) => build_dictionary_array(AI::UInt32, indices, *arr.values),
                A::UInt64(indices) => build_dictionary_array(AI::UInt64, indices, *arr.values),
                // TODO: improve error message by including the data type
                _ => fail!(
                    ErrorKind::Unsupported,
                    "unsupported dictionary index array during arrow2 conversion"
                ),
            },
            A::List(arr) => build_list_array(
                AT::List,
                arr.offsets,
                arr.meta,
                (*arr.element).try_into()?,
                arr.validity,
            ),
            A::LargeList(arr) => build_list_array(
                AT::LargeList,
                arr.offsets,
                arr.meta,
                (*arr.element).try_into()?,
                arr.validity,
            ),
            A::Struct(arr) => {
                let mut values = Vec::new();
                let mut fields = Vec::new();

                for (child, meta) in arr.fields {
                    let child: Box<dyn arrow2::array::Array> = child.try_into()?;
                    let field = field_from_array_and_meta(child.as_ref(), meta);

                    values.push(child);
                    fields.push(field);
                }
                Ok(Box::new(arrow2::array::StructArray::new(
                    AT::Struct(fields),
                    values,
                    arr.validity
                        .map(|v| arrow2::bitmap::Bitmap::from_u8_vec(v, arr.len)),
                )))
            }
            A::Map(arr) => {
                let child: Box<dyn arrow2::array::Array> = (*arr.element).try_into()?;
                let field = field_from_array_and_meta(child.as_ref(), arr.meta);
                let validity = arr.validity.map(|v| {
                    arrow2::bitmap::Bitmap::from_u8_vec(v, arr.offsets.len().saturating_sub(1))
                });
                Ok(Box::new(arrow2::array::MapArray::new(
                    AT::Map(Box::new(field), false),
                    arr.offsets.try_into()?,
                    child,
                    validity,
                )))
            }
            A::DenseUnion(arr) => {
                let mut values = Vec::new();
                let mut fields = Vec::new();
                let mut type_ids = Vec::new();

                for (type_id, child, meta) in arr.fields {
                    let child: Box<dyn arrow2::array::Array> = child.try_into()?;
                    let field = field_from_array_and_meta(child.as_ref(), meta);

                    type_ids.push(type_id.into());
                    values.push(child);
                    fields.push(field);
                }

                Ok(Box::new(arrow2::array::UnionArray::try_new(
                    AT::Union(fields, Some(type_ids), arrow2::datatypes::UnionMode::Dense),
                    arr.types.into(),
                    values,
                    Some(arr.offsets.into()),
                )?))
            }
            A::FixedSizeList(arr) => {
                let child: Box<dyn arrow2::array::Array> = (*arr.element).try_into()?;
                let child_field = field_from_array_and_meta(child.as_ref(), arr.meta);
                let data_type = AT::FixedSizeList(Box::new(child_field), arr.n.try_into()?);
                let validity = arr
                    .validity
                    .map(|v| arrow2::bitmap::Bitmap::from_u8_vec(v, arr.len));

                Ok(Box::new(arrow2::array::FixedSizeListArray::try_new(
                    data_type, child, validity,
                )?))
            }
            A::FixedSizeBinary(arr) => {
                let n = usize::try_from(arr.n)?;
                let len = arr.data.len() / n;
                let validity = arr
                    .validity
                    .map(|v| arrow2::bitmap::Bitmap::from_u8_vec(v, len));

                Ok(Box::new(arrow2::array::FixedSizeBinaryArray::try_new(
                    AT::FixedSizeBinary(n),
                    arrow2::buffer::Buffer::from(arr.data),
                    validity,
                )?))
            }
        }
    }
}

fn build_primitive_array<T: arrow2::types::NativeType>(
    data_type: arrow2::datatypes::DataType,
    buffer: Vec<T>,
    validity: Option<Vec<u8>>,
) -> Result<Box<dyn arrow2::array::Array>> {
    let validity = validity.map(|v| arrow2::bitmap::Bitmap::from_u8_vec(v, buffer.len()));
    let buffer = arrow2::buffer::Buffer::from(buffer);
    Ok(Box::new(arrow2::array::PrimitiveArray::try_new(
        data_type, buffer, validity,
    )?))
}

fn build_utf8_array<O: arrow2::types::Offset>(
    data_type: arrow2::datatypes::DataType,
    offsets: Vec<O>,
    data: Vec<u8>,
    validity: Option<Vec<u8>>,
) -> Result<Box<dyn arrow2::array::Array>> {
    let validity =
        validity.map(|v| arrow2::bitmap::Bitmap::from_u8_vec(v, offsets.len().saturating_sub(1)));
    Ok(Box::new(arrow2::array::Utf8Array::new(
        data_type,
        offsets.try_into()?,
        arrow2::buffer::Buffer::from(data),
        validity,
    )))
}

fn build_binary_array<O: arrow2::types::Offset>(
    data_type: arrow2::datatypes::DataType,
    offsets: Vec<O>,
    data: Vec<u8>,
    validity: Option<Vec<u8>>,
) -> Result<Box<dyn arrow2::array::Array>> {
    let validity =
        validity.map(|v| arrow2::bitmap::Bitmap::from_u8_vec(v, offsets.len().saturating_sub(1)));
    Ok(Box::new(arrow2::array::BinaryArray::new(
        data_type,
        offsets.try_into()?,
        arrow2::buffer::Buffer::from(data),
        validity,
    )))
}

fn build_list_array<
    F: FnOnce(Box<arrow2::datatypes::Field>) -> arrow2::datatypes::DataType,
    O: arrow2::types::Offset,
>(
    data_type: F,
    offsets: Vec<O>,
    meta: FieldMeta,
    values: Box<dyn arrow2::array::Array>,
    validity: Option<Vec<u8>>,
) -> Result<Box<dyn arrow2::array::Array>> {
    let validity =
        validity.map(|v| arrow2::bitmap::Bitmap::from_u8_vec(v, offsets.len().saturating_sub(1)));
    Ok(Box::new(arrow2::array::ListArray::new(
        data_type(Box::new(field_from_array_and_meta(values.as_ref(), meta))),
        offsets.try_into()?,
        values,
        validity,
    )))
}

fn field_from_array_and_meta(
    arr: &dyn arrow2::array::Array,
    meta: FieldMeta,
) -> arrow2::datatypes::Field {
    arrow2::datatypes::Field::new(meta.name, arr.data_type().clone(), meta.nullable)
        .with_metadata(meta.metadata.into_iter().collect())
}

fn build_dictionary_array<K: arrow2::array::DictionaryKey>(
    indices_type: arrow2::datatypes::IntegerType,
    indices: PrimitiveArray<K>,
    values: Array,
) -> Result<Box<dyn arrow2::array::Array>> {
    let values: Box<dyn arrow2::array::Array> = values.try_into()?;
    let validity = indices
        .validity
        .map(|v| arrow2::bitmap::Bitmap::from_u8_vec(v, indices.values.len()));
    let keys =
        arrow2::array::PrimitiveArray::new(indices_type.into(), indices.values.into(), validity);

    Ok(Box::new(arrow2::array::DictionaryArray::try_new(
        arrow2::datatypes::DataType::Dictionary(
            indices_type,
            Box::new(values.data_type().clone()),
            false,
        ),
        keys,
        values,
    )?))
}

/// Conversion from `arrow2` arrays
impl<'a> TryFrom<&'a dyn arrow2::array::Array> for View<'a> {
    type Error = MarrowError;

    fn try_from(array: &'a dyn arrow2::array::Array) -> Result<Self> {
        use {arrow2::datatypes::DataType as AT, View as V};

        use arrow2::array::Array as _;

        let any = array.as_any();
        if let Some(array) = any.downcast_ref::<arrow2::array::NullArray>() {
            Ok(V::Null(NullView { len: array.len() }))
        } else if let Some(array) = any.downcast_ref::<arrow2::array::BooleanArray>() {
            let (values_data, values_offset, _) = array.values().as_slice();
            Ok(V::Boolean(BooleanView {
                len: array.len(),
                validity: bits_with_offset_from_bitmap(array.validity()),
                values: BitsWithOffset {
                    offset: values_offset,
                    data: values_data,
                },
            }))
        } else if let Some(array) = any.downcast_ref::<arrow2::array::PrimitiveArray<i8>>() {
            Ok(V::Int8(view_primitive_array(array)))
        } else if let Some(array) = any.downcast_ref::<arrow2::array::PrimitiveArray<i16>>() {
            Ok(V::Int16(view_primitive_array(array)))
        } else if let Some(array) = any.downcast_ref::<arrow2::array::PrimitiveArray<i32>>() {
            match array.data_type() {
                AT::Int32 => Ok(V::Int32(view_primitive_array(array))),
                AT::Date32 => Ok(V::Date32(view_primitive_array(array))),
                AT::Time32(unit) => Ok(V::Time32(TimeView {
                    unit: (*unit).into(),
                    validity: bits_with_offset_from_bitmap(array.validity()),
                    values: array.values().as_slice(),
                })),
                dt => fail!(
                    ErrorKind::Unsupported,
                    "unsupported data type {dt:?} for i32 arrow2 array"
                ),
            }
        } else if let Some(array) = any.downcast_ref::<arrow2::array::PrimitiveArray<i64>>() {
            match array.data_type() {
                AT::Int64 => Ok(V::Int64(view_primitive_array(array))),
                AT::Date64 => Ok(V::Date64(view_primitive_array(array))),
                AT::Timestamp(unit, tz) => Ok(V::Timestamp(TimestampView {
                    unit: (*unit).into(),
                    timezone: tz.to_owned(),
                    validity: bits_with_offset_from_bitmap(array.validity()),
                    values: array.values().as_slice(),
                })),
                AT::Time64(unit) => Ok(V::Time64(TimeView {
                    unit: (*unit).into(),
                    validity: bits_with_offset_from_bitmap(array.validity()),
                    values: array.values().as_slice(),
                })),
                AT::Duration(unit) => Ok(V::Duration(TimeView {
                    unit: (*unit).into(),
                    validity: bits_with_offset_from_bitmap(array.validity()),
                    values: array.values().as_slice(),
                })),
                dt => fail!(
                    ErrorKind::Unsupported,
                    "unsupported data type {dt:?} for i64 arrow2 array"
                ),
            }
        } else if let Some(array) = any.downcast_ref::<arrow2::array::PrimitiveArray<i128>>() {
            match array.data_type() {
                AT::Decimal(precision, scale) => Ok(V::Decimal128(DecimalView {
                    precision: (*precision).try_into()?,
                    scale: (*scale).try_into()?,
                    validity: bits_with_offset_from_bitmap(array.validity()),
                    values: array.values().as_slice(),
                })),
                dt => fail!(
                    ErrorKind::Unsupported,
                    "unsupported data type {dt:?} for i128 arrow2 array"
                ),
            }
        } else if let Some(array) = any.downcast_ref::<arrow2::array::PrimitiveArray<u8>>() {
            Ok(V::UInt8(view_primitive_array(array)))
        } else if let Some(array) = any.downcast_ref::<arrow2::array::PrimitiveArray<u16>>() {
            Ok(V::UInt16(view_primitive_array(array)))
        } else if let Some(array) = any.downcast_ref::<arrow2::array::PrimitiveArray<u32>>() {
            Ok(V::UInt32(view_primitive_array(array)))
        } else if let Some(array) = any.downcast_ref::<arrow2::array::PrimitiveArray<u64>>() {
            Ok(V::UInt64(view_primitive_array(array)))
        } else if let Some(array) =
            any.downcast_ref::<arrow2::array::PrimitiveArray<arrow2::types::f16>>()
        {
            Ok(V::Float16(PrimitiveView {
                values: bytemuck::cast_slice::<arrow2::types::f16, half::f16>(
                    array.values().as_slice(),
                ),
                validity: bits_with_offset_from_bitmap(array.validity()),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow2::array::PrimitiveArray<f32>>() {
            Ok(V::Float32(view_primitive_array(array)))
        } else if let Some(array) = any.downcast_ref::<arrow2::array::PrimitiveArray<f64>>() {
            Ok(V::Float64(view_primitive_array(array)))
        } else if let Some(array) = any.downcast_ref::<arrow2::array::Utf8Array<i32>>() {
            Ok(V::Utf8(BytesView {
                validity: bits_with_offset_from_bitmap(array.validity()),
                offsets: array.offsets().as_slice(),
                data: array.values().as_slice(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow2::array::Utf8Array<i64>>() {
            Ok(V::LargeUtf8(BytesView {
                validity: bits_with_offset_from_bitmap(array.validity()),
                offsets: array.offsets().as_slice(),
                data: array.values().as_slice(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow2::array::BinaryArray<i32>>() {
            Ok(V::Binary(BytesView {
                validity: bits_with_offset_from_bitmap(array.validity()),
                offsets: array.offsets().as_slice(),
                data: array.values().as_slice(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow2::array::BinaryArray<i64>>() {
            Ok(V::LargeBinary(BytesView {
                validity: bits_with_offset_from_bitmap(array.validity()),
                offsets: array.offsets().as_slice(),
                data: array.values().as_slice(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow2::array::DictionaryArray<i8>>() {
            Ok(V::Dictionary(view_dictionary_array(V::Int8, array)?))
        } else if let Some(array) = any.downcast_ref::<arrow2::array::DictionaryArray<i16>>() {
            Ok(V::Dictionary(view_dictionary_array(V::Int16, array)?))
        } else if let Some(array) = any.downcast_ref::<arrow2::array::DictionaryArray<i32>>() {
            Ok(V::Dictionary(view_dictionary_array(V::Int32, array)?))
        } else if let Some(array) = any.downcast_ref::<arrow2::array::DictionaryArray<i64>>() {
            Ok(V::Dictionary(view_dictionary_array(V::Int64, array)?))
        } else if let Some(array) = any.downcast_ref::<arrow2::array::DictionaryArray<u8>>() {
            Ok(V::Dictionary(view_dictionary_array(V::UInt8, array)?))
        } else if let Some(array) = any.downcast_ref::<arrow2::array::DictionaryArray<u16>>() {
            Ok(V::Dictionary(view_dictionary_array(V::UInt16, array)?))
        } else if let Some(array) = any.downcast_ref::<arrow2::array::DictionaryArray<u32>>() {
            Ok(V::Dictionary(view_dictionary_array(V::UInt32, array)?))
        } else if let Some(array) = any.downcast_ref::<arrow2::array::DictionaryArray<u64>>() {
            Ok(V::Dictionary(view_dictionary_array(V::UInt64, array)?))
        } else if let Some(array) = any.downcast_ref::<arrow2::array::ListArray<i32>>() {
            let AT::List(field) = array.data_type() else {
                fail!(
                    ErrorKind::Unsupported,
                    "invalid data type for arrow2 List array: {:?}",
                    array.data_type()
                );
            };
            Ok(V::List(ListView {
                meta: meta_from_field(field.as_ref().try_into()?),
                validity: bits_with_offset_from_bitmap(array.validity()),
                offsets: array.offsets().as_slice(),
                element: Box::new(array.values().as_ref().try_into()?),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow2::array::ListArray<i64>>() {
            let AT::LargeList(field) = array.data_type() else {
                fail!(
                    ErrorKind::Unsupported,
                    "invalid data type for arrow2 LargeList array: {:?}",
                    array.data_type()
                );
            };
            Ok(V::LargeList(ListView {
                meta: meta_from_field(field.as_ref().try_into()?),
                validity: bits_with_offset_from_bitmap(array.validity()),
                offsets: array.offsets().as_slice(),
                element: Box::new(array.values().as_ref().try_into()?),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow2::array::StructArray>() {
            let AT::Struct(child_fields) = array.data_type() else {
                fail!(
                    ErrorKind::Unsupported,
                    "invalid data type for arrow2 Struct array: {:?}",
                    array.data_type()
                );
            };
            let mut fields = Vec::new();
            for (child_field, child) in child_fields.iter().zip(array.values()) {
                fields.push((
                    child.as_ref().try_into()?,
                    meta_from_field(child_field.try_into()?),
                ));
            }
            Ok(V::Struct(StructView {
                len: array.len(),
                validity: bits_with_offset_from_bitmap(array.validity()),
                fields,
            }))
        } else if let Some(array) = any.downcast_ref::<arrow2::array::MapArray>() {
            let AT::Map(field, _) = array.data_type() else {
                fail!(
                    ErrorKind::Unsupported,
                    "invalid data type for arrow2 Map array: {:?}",
                    array.data_type(),
                );
            };
            let meta = meta_from_field(field.as_ref().try_into()?);
            let element: View<'_> = array.field().as_ref().try_into()?;

            Ok(V::Map(ListView {
                element: Box::new(element),
                meta,
                validity: bits_with_offset_from_bitmap(array.validity()),
                offsets: array.offsets().as_slice(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow2::array::UnionArray>() {
            let AT::Union(union_fields, type_ids, arrow2::datatypes::UnionMode::Dense) =
                array.data_type()
            else {
                fail!(
                    ErrorKind::Unsupported,
                    "Invalid data type: only dense unions are supported"
                );
            };

            let type_ids = if let Some(type_ids) = type_ids.as_ref() {
                Cow::Borrowed(type_ids)
            } else {
                let mut type_ids = Vec::new();
                for idx in 0..union_fields.len() {
                    type_ids.push(idx.try_into()?);
                }
                Cow::Owned(type_ids)
            };

            let types = array.types().as_slice();
            let Some(offsets) = array.offsets() else {
                fail!(
                    ErrorKind::Unsupported,
                    "DenseUnion array without offsets are not supported"
                );
            };

            let mut fields = Vec::new();
            for ((type_id, child), child_field) in
                type_ids.iter().zip(array.fields().iter()).zip(union_fields)
            {
                fields.push((
                    (*type_id).try_into()?,
                    child.as_ref().try_into()?,
                    meta_from_field(child_field.try_into()?),
                ));
            }

            Ok(V::DenseUnion(DenseUnionView {
                types,
                offsets: offsets.as_slice(),
                fields,
            }))
        } else if let Some(array) = any.downcast_ref::<arrow2::array::FixedSizeListArray>() {
            let AT::FixedSizeList(field, _) = array.data_type() else {
                fail!(
                    ErrorKind::Unsupported,
                    "Invalid type: expected FixedSizeList"
                );
            };

            let child_view: View<'_> = array.values().as_ref().try_into()?;

            Ok(V::FixedSizeList(FixedSizeListView {
                len: array.len(),
                n: array.size().try_into()?,
                validity: bits_with_offset_from_bitmap(array.validity()),
                meta: meta_from_field(field.as_ref().try_into()?),
                element: Box::new(child_view),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow2::array::FixedSizeBinaryArray>() {
            Ok(V::FixedSizeBinary(FixedSizeBinaryView {
                n: array.size().try_into()?,
                validity: bits_with_offset_from_bitmap(array.validity()),
                data: array.values().as_slice(),
            }))
        } else {
            fail!(
                ErrorKind::Unsupported,
                "Cannot convert array with data type {:?} into an array view",
                array.data_type()
            );
        }
    }
}

fn view_primitive_array<T: arrow2::types::NativeType>(
    array: &arrow2::array::PrimitiveArray<T>,
) -> PrimitiveView<'_, T> {
    PrimitiveView {
        values: array.values().as_slice(),
        validity: bits_with_offset_from_bitmap(array.validity()),
    }
}

fn view_dictionary_array<
    'a,
    K: arrow2::array::DictionaryKey,
    I: FnOnce(PrimitiveView<'a, K>) -> View<'a>,
>(
    index_type: I,
    array: &'a arrow2::array::DictionaryArray<K>,
) -> Result<DictionaryView<'a>> {
    Ok(DictionaryView {
        indices: Box::new(index_type(view_primitive_array(array.keys()))),
        values: Box::new(array.values().as_ref().try_into()?),
    })
}

fn bits_with_offset_from_bitmap(
    bitmap: Option<&arrow2::bitmap::Bitmap>,
) -> Option<BitsWithOffset<'_>> {
    let (data, offset, _) = bitmap?.as_slice();
    Some(BitsWithOffset { data, offset })
}
