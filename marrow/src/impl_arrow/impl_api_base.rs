// the implementation
use std::sync::Arc;

use half::f16;

use crate::{
    array::Array,
    datatypes::{DataType, Field, TimeUnit, UnionMode},
    error::{fail, ErrorKind, MarrowError, Result},
    meta::{meta_from_field, FieldMeta},
    view::{
        BitsWithOffset, BooleanView, BytesView, DecimalView, DenseUnionView, DictionaryView,
        FixedSizeListView, ListView, NullView, PrimitiveView, StructView, TimeView, TimestampView,
        View,
    },
};

impl From<arrow_schema::ArrowError> for MarrowError {
    fn from(err: arrow_schema::ArrowError) -> Self {
        crate::error::error_with_kind_message_cause(ErrorKind::ArrowError, err.to_string(), err)
    }
}

/// Converison from `arrow` data types (*requires one of the `arrow-{version}` features*)
// only some arrow version implement Copy for unit
#[allow(clippy::clone_on_copy)]
impl TryFrom<&arrow_schema::DataType> for DataType {
    type Error = MarrowError;

    fn try_from(value: &arrow_schema::DataType) -> Result<DataType> {
        use {arrow_schema::DataType as AT, DataType as T, Field as F};
        match value {
            AT::Boolean => Ok(T::Boolean),
            AT::Null => Ok(T::Null),
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
            AT::Utf8 => Ok(T::Utf8),
            AT::LargeUtf8 => Ok(T::LargeUtf8),
            AT::Date32 => Ok(T::Date32),
            AT::Date64 => Ok(T::Date64),
            AT::Decimal128(precision, scale) => Ok(T::Decimal128(*precision, *scale)),
            AT::Time32(unit) => Ok(T::Time32(unit.clone().try_into()?)),
            AT::Time64(unit) => Ok(T::Time64(unit.clone().try_into()?)),
            AT::Timestamp(unit, tz) => Ok(T::Timestamp(
                unit.clone().try_into()?,
                tz.as_ref().map(|s| s.to_string()),
            )),
            AT::Duration(unit) => Ok(T::Duration(unit.clone().try_into()?)),
            AT::Binary => Ok(T::Binary),
            AT::LargeBinary => Ok(T::LargeBinary),
            AT::FixedSizeBinary(n) => Ok(T::FixedSizeBinary(*n)),
            AT::List(field) => Ok(T::List(F::try_from(field.as_ref())?.into())),
            AT::LargeList(field) => Ok(T::LargeList(F::try_from(field.as_ref())?.into())),
            AT::FixedSizeList(field, n) => {
                Ok(T::FixedSizeList(F::try_from(field.as_ref())?.into(), *n))
            }
            AT::Map(field, sorted) => Ok(T::Map(F::try_from(field.as_ref())?.into(), *sorted)),
            AT::Struct(in_fields) => {
                let mut fields = Vec::new();
                for field in in_fields {
                    fields.push(field.as_ref().try_into()?);
                }
                Ok(T::Struct(fields))
            }
            AT::Dictionary(key, value) => Ok(T::Dictionary(
                T::try_from(key.as_ref())?.into(),
                T::try_from(value.as_ref())?.into(),
                false,
            )),
            AT::Union(in_fields, mode) => {
                let mut fields = Vec::new();
                for (type_id, field) in in_fields.iter() {
                    fields.push((type_id, F::try_from(field.as_ref())?));
                }
                Ok(T::Union(fields, (*mode).try_into()?))
            }
            data_type => fail!(
                ErrorKind::Unsupported,
                "Unsupported arrow data type {data_type}"
            ),
        }
    }
}

/// Converison from `arrow` fields (*requires one of the `arrow-{version}` features*)
impl TryFrom<&arrow_schema::Field> for Field {
    type Error = MarrowError;

    fn try_from(field: &arrow_schema::Field) -> Result<Self> {
        Ok(Field {
            name: field.name().to_owned(),
            data_type: DataType::try_from(field.data_type())?,
            metadata: field.metadata().clone(),
            nullable: field.is_nullable(),
        })
    }
}

/// Converison to `arrow` data types (*requires one of the `arrow-{version}` features*)
impl TryFrom<&DataType> for arrow_schema::DataType {
    type Error = MarrowError;

    fn try_from(value: &DataType) -> std::result::Result<Self, Self::Error> {
        use {arrow_schema::DataType as AT, arrow_schema::Field as AF, DataType as T};
        match value {
            T::Boolean => Ok(AT::Boolean),
            T::Null => Ok(AT::Null),
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
            T::Utf8 => Ok(AT::Utf8),
            T::LargeUtf8 => Ok(AT::LargeUtf8),
            T::Date32 => Ok(AT::Date32),
            T::Date64 => Ok(AT::Date64),
            T::Decimal128(precision, scale) => Ok(AT::Decimal128(*precision, *scale)),
            T::Time32(unit) => Ok(AT::Time32((*unit).try_into()?)),
            T::Time64(unit) => Ok(AT::Time64((*unit).try_into()?)),
            T::Timestamp(unit, tz) => Ok(AT::Timestamp(
                (*unit).try_into()?,
                tz.as_ref().map(|s| s.to_string().into()),
            )),
            T::Duration(unit) => Ok(AT::Duration((*unit).try_into()?)),
            T::Binary => Ok(AT::Binary),
            T::LargeBinary => Ok(AT::LargeBinary),
            T::FixedSizeBinary(n) => Ok(AT::FixedSizeBinary(*n)),
            T::List(field) => Ok(AT::List(AF::try_from(field.as_ref())?.into())),
            T::LargeList(field) => Ok(AT::LargeList(AF::try_from(field.as_ref())?.into())),
            T::FixedSizeList(field, n) => {
                Ok(AT::FixedSizeList(AF::try_from(field.as_ref())?.into(), *n))
            }
            T::Map(field, sorted) => Ok(AT::Map(AF::try_from(field.as_ref())?.into(), *sorted)),
            T::Struct(in_fields) => {
                let mut fields: Vec<arrow_schema::FieldRef> = Vec::new();
                for field in in_fields {
                    fields.push(AF::try_from(field)?.into());
                }
                Ok(AT::Struct(fields.into()))
            }
            T::Dictionary(key, value, _sorted) => Ok(AT::Dictionary(
                AT::try_from(key.as_ref())?.into(),
                AT::try_from(value.as_ref())?.into(),
            )),
            T::Union(in_fields, mode) => {
                let mut fields = Vec::new();
                for (type_id, field) in in_fields {
                    fields.push((*type_id, Arc::new(AF::try_from(field)?)));
                }
                Ok(AT::Union(fields.into_iter().collect(), (*mode).try_into()?))
            }
        }
    }
}

/// Converison to `arrow` fields (*requires one of the `arrow-{version}` features*)
impl TryFrom<&Field> for arrow_schema::Field {
    type Error = MarrowError;

    fn try_from(value: &Field) -> Result<Self> {
        let mut field = arrow_schema::Field::new(
            &value.name,
            arrow_schema::DataType::try_from(&value.data_type)?,
            value.nullable,
        );
        field.set_metadata(value.metadata.clone());
        Ok(field)
    }
}

/// Conversion to `arrow` time units (*requires one of the `arrow-{version}` features*)
impl TryFrom<TimeUnit> for arrow_schema::TimeUnit {
    type Error = MarrowError;

    fn try_from(value: TimeUnit) -> Result<arrow_schema::TimeUnit> {
        match value {
            TimeUnit::Second => Ok(arrow_schema::TimeUnit::Second),
            TimeUnit::Millisecond => Ok(arrow_schema::TimeUnit::Millisecond),
            TimeUnit::Microsecond => Ok(arrow_schema::TimeUnit::Microsecond),
            TimeUnit::Nanosecond => Ok(arrow_schema::TimeUnit::Nanosecond),
        }
    }
}

/// Conversion from `arrow` time units (*requires one of the `arrow-{version}` features*)
impl TryFrom<arrow_schema::TimeUnit> for TimeUnit {
    type Error = MarrowError;

    fn try_from(value: arrow_schema::TimeUnit) -> Result<TimeUnit> {
        match value {
            arrow_schema::TimeUnit::Second => Ok(TimeUnit::Second),
            arrow_schema::TimeUnit::Millisecond => Ok(TimeUnit::Millisecond),
            arrow_schema::TimeUnit::Microsecond => Ok(TimeUnit::Microsecond),
            arrow_schema::TimeUnit::Nanosecond => Ok(TimeUnit::Nanosecond),
        }
    }
}

/// Conversion from `arrow` union modes (*requires one of the `arrow-{version}` features*)
impl TryFrom<arrow_schema::UnionMode> for UnionMode {
    type Error = MarrowError;

    fn try_from(value: arrow_schema::UnionMode) -> Result<Self> {
        match value {
            arrow_schema::UnionMode::Dense => Ok(UnionMode::Dense),
            arrow_schema::UnionMode::Sparse => Ok(UnionMode::Sparse),
        }
    }
}

/// Conversion to `arrow` union modes (*requires one of the `arrow-{version}` features*)
impl TryFrom<UnionMode> for arrow_schema::UnionMode {
    type Error = MarrowError;

    fn try_from(value: UnionMode) -> Result<Self> {
        match value {
            UnionMode::Dense => Ok(arrow_schema::UnionMode::Dense),
            UnionMode::Sparse => Ok(arrow_schema::UnionMode::Sparse),
        }
    }
}

/// Converison to `arrow` arrays (*requires one of the `arrow-{version}` features*)
impl TryFrom<Array> for Arc<dyn arrow_array::Array> {
    type Error = MarrowError;

    fn try_from(value: Array) -> Result<Arc<dyn arrow_array::Array>> {
        Ok(arrow_array::make_array(build_array_data(value)?))
    }
}

fn build_array_data(value: Array) -> Result<arrow_data::ArrayData> {
    use Array as A;
    type ArrowF16 =
        <arrow_array::types::Float16Type as arrow_array::types::ArrowPrimitiveType>::Native;

    fn f16_to_f16(v: f16) -> ArrowF16 {
        ArrowF16::from_bits(v.to_bits())
    }

    match value {
        A::Null(arr) => {
            use arrow_array::Array;
            Ok(arrow_array::NullArray::new(arr.len).into_data())
        }
        A::Boolean(arr) => Ok(arrow_data::ArrayData::try_new(
            arrow_schema::DataType::Boolean,
            // NOTE: use the explicit len
            arr.len,
            arr.validity.map(arrow_buffer::Buffer::from_vec),
            0,
            vec![arrow_buffer::ScalarBuffer::from(arr.values).into_inner()],
            vec![],
        )?),
        A::Int8(arr) => primitive_into_data(arrow_schema::DataType::Int8, arr.validity, arr.values),
        A::Int16(arr) => {
            primitive_into_data(arrow_schema::DataType::Int16, arr.validity, arr.values)
        }
        A::Int32(arr) => {
            primitive_into_data(arrow_schema::DataType::Int32, arr.validity, arr.values)
        }
        A::Int64(arr) => {
            primitive_into_data(arrow_schema::DataType::Int64, arr.validity, arr.values)
        }
        A::UInt8(arr) => {
            primitive_into_data(arrow_schema::DataType::UInt8, arr.validity, arr.values)
        }
        A::UInt16(arr) => {
            primitive_into_data(arrow_schema::DataType::UInt16, arr.validity, arr.values)
        }
        A::UInt32(arr) => {
            primitive_into_data(arrow_schema::DataType::UInt32, arr.validity, arr.values)
        }
        A::UInt64(arr) => {
            primitive_into_data(arrow_schema::DataType::UInt64, arr.validity, arr.values)
        }
        A::Float16(arr) => primitive_into_data(
            arrow_schema::DataType::Float16,
            arr.validity,
            arr.values.into_iter().map(f16_to_f16).collect(),
        ),
        A::Float32(arr) => {
            primitive_into_data(arrow_schema::DataType::Float32, arr.validity, arr.values)
        }
        A::Float64(arr) => {
            primitive_into_data(arrow_schema::DataType::Float64, arr.validity, arr.values)
        }
        A::Date32(arr) => {
            primitive_into_data(arrow_schema::DataType::Date32, arr.validity, arr.values)
        }
        A::Date64(arr) => {
            primitive_into_data(arrow_schema::DataType::Date64, arr.validity, arr.values)
        }
        A::Timestamp(arr) => primitive_into_data(
            arrow_schema::DataType::Timestamp(arr.unit.try_into()?, arr.timezone.map(String::into)),
            arr.validity,
            arr.values,
        ),
        A::Time32(arr) => primitive_into_data(
            arrow_schema::DataType::Time32(arr.unit.try_into()?),
            arr.validity,
            arr.values,
        ),
        A::Time64(arr) => primitive_into_data(
            arrow_schema::DataType::Time64(arr.unit.try_into()?),
            arr.validity,
            arr.values,
        ),
        A::Duration(arr) => primitive_into_data(
            arrow_schema::DataType::Duration(arr.unit.try_into()?),
            arr.validity,
            arr.values,
        ),
        A::Decimal128(arr) => primitive_into_data(
            arrow_schema::DataType::Decimal128(arr.precision, arr.scale),
            arr.validity,
            arr.values,
        ),
        A::Utf8(arr) => bytes_into_data(
            arrow_schema::DataType::Utf8,
            arr.offsets,
            arr.data,
            arr.validity,
        ),
        A::LargeUtf8(arr) => bytes_into_data(
            arrow_schema::DataType::LargeUtf8,
            arr.offsets,
            arr.data,
            arr.validity,
        ),
        A::Binary(arr) => bytes_into_data(
            arrow_schema::DataType::Binary,
            arr.offsets,
            arr.data,
            arr.validity,
        ),
        A::LargeBinary(arr) => bytes_into_data(
            arrow_schema::DataType::LargeBinary,
            arr.offsets,
            arr.data,
            arr.validity,
        ),
        A::Struct(arr) => {
            let mut fields = Vec::new();
            let mut data = Vec::new();

            for (field, meta) in arr.fields {
                let child = build_array_data(field)?;
                fields.push(Arc::new(field_from_data_and_meta(&child, meta)));
                data.push(child);
            }
            let data_type = arrow_schema::DataType::Struct(fields.into());

            Ok(arrow_data::ArrayData::builder(data_type)
                .len(arr.len)
                .null_bit_buffer(arr.validity.map(arrow_buffer::Buffer::from_vec))
                .child_data(data)
                .build()?)
        }
        A::List(arr) => {
            let child = build_array_data(*arr.elements)?;
            let field = field_from_data_and_meta(&child, arr.meta);
            list_into_data(
                arrow_schema::DataType::List(Arc::new(field)),
                arr.offsets.len().saturating_sub(1),
                arr.offsets,
                child,
                arr.validity,
            )
        }
        A::LargeList(arr) => {
            let child = build_array_data(*arr.elements)?;
            let field = field_from_data_and_meta(&child, arr.meta);
            list_into_data(
                arrow_schema::DataType::LargeList(Arc::new(field)),
                arr.offsets.len().saturating_sub(1),
                arr.offsets,
                child,
                arr.validity,
            )
        }
        A::FixedSizeList(arr) => {
            let child = build_array_data(*arr.elements)?;
            if (child.len() % usize::try_from(arr.n)?) != 0 {
                fail!(
                    ErrorKind::Unsupported,
                    "Invalid FixedSizeList: number of child elements ({}) not divisible by n ({})",
                    child.len(),
                    arr.n,
                );
            }
            let field = field_from_data_and_meta(&child, arr.meta);
            Ok(arrow_data::ArrayData::try_new(
                arrow_schema::DataType::FixedSizeList(Arc::new(field), arr.n),
                child.len() / usize::try_from(arr.n)?,
                arr.validity.map(arrow_buffer::Buffer::from_vec),
                0,
                vec![],
                vec![child],
            )?)
        }
        A::FixedSizeBinary(arr) => {
            if (arr.data.len() % usize::try_from(arr.n)?) != 0 {
                fail!(
                    ErrorKind::Unsupported,
                    "Invalid FixedSizeBinary: number of child elements ({}) not divisible by n ({})",
                    arr.data.len(),
                    arr.n,
                );
            }
            Ok(arrow_data::ArrayData::try_new(
                arrow_schema::DataType::FixedSizeBinary(arr.n),
                arr.data.len() / usize::try_from(arr.n)?,
                arr.validity.map(arrow_buffer::Buffer::from_vec),
                0,
                vec![arrow_buffer::ScalarBuffer::from(arr.data).into_inner()],
                vec![],
            )?)
        }
        A::Dictionary(arr) => {
            let indices = build_array_data(*arr.indices)?;
            let values = build_array_data(*arr.values)?;
            let data_type = arrow_schema::DataType::Dictionary(
                Box::new(indices.data_type().clone()),
                Box::new(values.data_type().clone()),
            );

            Ok(indices
                .into_builder()
                .data_type(data_type)
                .child_data(vec![values])
                .build()?)
        }
        A::Map(arr) => {
            let child = build_array_data(*arr.elements)?;
            let field = field_from_data_and_meta(&child, arr.meta);
            Ok(arrow_data::ArrayData::try_new(
                arrow_schema::DataType::Map(Arc::new(field), false),
                arr.offsets.len().saturating_sub(1),
                arr.validity.map(arrow_buffer::Buffer::from_vec),
                0,
                vec![arrow_buffer::ScalarBuffer::from(arr.offsets).into_inner()],
                vec![child],
            )?)
        }
        A::DenseUnion(arr) => {
            let mut fields = Vec::new();
            let mut child_data = Vec::new();

            for (type_id, array, meta) in arr.fields {
                let child = build_array_data(array)?;
                let field = field_from_data_and_meta(&child, meta);

                fields.push((type_id, Arc::new(field)));
                child_data.push(child);
            }

            Ok(arrow_data::ArrayData::try_new(
                arrow_schema::DataType::Union(
                    fields.into_iter().collect(),
                    arrow_schema::UnionMode::Dense,
                ),
                arr.types.len(),
                None,
                0,
                vec![
                    arrow_buffer::ScalarBuffer::from(arr.types).into_inner(),
                    arrow_buffer::ScalarBuffer::from(arr.offsets).into_inner(),
                ],
                child_data,
            )?)
        }
    }
}

/// Converison from `arrow` arrays (*requires one of the `arrow-{version}` features*)
impl<'a> TryFrom<&'a dyn arrow_array::Array> for View<'a> {
    type Error = MarrowError;

    fn try_from(array: &'a dyn arrow_array::Array) -> Result<Self> {
        use arrow_array::Array;

        let any = array.as_any();
        if let Some(array) = any.downcast_ref::<arrow_array::NullArray>() {
            use arrow_array::Array;

            Ok(View::Null(NullView { len: array.len() }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::BooleanArray>() {
            Ok(View::Boolean(BooleanView {
                len: array.len(),
                validity: get_bits_with_offset(array),
                values: BitsWithOffset {
                    offset: array.values().offset(),
                    data: array.values().values(),
                },
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::Int8Array>() {
            Ok(View::Int8(PrimitiveView {
                validity: get_bits_with_offset(array),
                values: array.values(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::Int16Array>() {
            Ok(View::Int16(PrimitiveView {
                validity: get_bits_with_offset(array),
                values: array.values(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::Int32Array>() {
            Ok(View::Int32(PrimitiveView {
                validity: get_bits_with_offset(array),
                values: array.values(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::Int64Array>() {
            Ok(View::Int64(PrimitiveView {
                validity: get_bits_with_offset(array),
                values: array.values(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::UInt8Array>() {
            Ok(View::UInt8(PrimitiveView {
                validity: get_bits_with_offset(array),
                values: array.values(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::UInt16Array>() {
            Ok(View::UInt16(PrimitiveView {
                validity: get_bits_with_offset(array),
                values: array.values(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::UInt32Array>() {
            Ok(View::UInt32(PrimitiveView {
                validity: get_bits_with_offset(array),
                values: array.values(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::UInt64Array>() {
            Ok(View::UInt64(PrimitiveView {
                validity: get_bits_with_offset(array),
                values: array.values(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::Float16Array>() {
            Ok(View::Float16(PrimitiveView {
                validity: get_bits_with_offset(array),
                values: array.values(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::Float32Array>() {
            Ok(View::Float32(PrimitiveView {
                validity: get_bits_with_offset(array),
                values: array.values(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::Float64Array>() {
            Ok(View::Float64(PrimitiveView {
                validity: get_bits_with_offset(array),
                values: array.values(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::Decimal128Array>() {
            use arrow_array::Array;

            let &arrow_schema::DataType::Decimal128(precision, scale) = array.data_type() else {
                fail!(
                    ErrorKind::Unsupported,
                    "Invalid data type for Decimal128 array: {}",
                    array.data_type()
                );
            };
            Ok(View::Decimal128(DecimalView {
                precision,
                scale,
                validity: get_bits_with_offset(array),
                values: array.values(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::Date32Array>() {
            Ok(View::Date32(PrimitiveView {
                validity: get_bits_with_offset(array),
                values: array.values(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::Date64Array>() {
            Ok(View::Date64(PrimitiveView {
                validity: get_bits_with_offset(array),
                values: array.values(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::Time32MillisecondArray>() {
            Ok(View::Time32(TimeView {
                unit: TimeUnit::Millisecond,
                validity: get_bits_with_offset(array),
                values: array.values(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::Time32SecondArray>() {
            Ok(View::Time32(TimeView {
                unit: TimeUnit::Second,
                validity: get_bits_with_offset(array),
                values: array.values(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::Time64NanosecondArray>() {
            Ok(View::Time64(TimeView {
                unit: TimeUnit::Nanosecond,
                validity: get_bits_with_offset(array),
                values: array.values(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::Time64MicrosecondArray>() {
            Ok(View::Time64(TimeView {
                unit: TimeUnit::Microsecond,
                validity: get_bits_with_offset(array),
                values: array.values(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::TimestampNanosecondArray>() {
            Ok(View::Timestamp(TimestampView {
                unit: TimeUnit::Nanosecond,
                timezone: array.timezone().map(str::to_owned),
                validity: get_bits_with_offset(array),
                values: array.values(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::TimestampMicrosecondArray>() {
            Ok(View::Timestamp(TimestampView {
                unit: TimeUnit::Microsecond,
                timezone: array.timezone().map(str::to_owned),
                validity: get_bits_with_offset(array),
                values: array.values(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::TimestampMillisecondArray>() {
            Ok(View::Timestamp(TimestampView {
                unit: TimeUnit::Millisecond,
                timezone: array.timezone().map(str::to_owned),
                validity: get_bits_with_offset(array),
                values: array.values(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::TimestampSecondArray>() {
            Ok(View::Timestamp(TimestampView {
                unit: TimeUnit::Second,
                timezone: array.timezone().map(str::to_owned),
                validity: get_bits_with_offset(array),
                values: array.values(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::DurationNanosecondArray>() {
            Ok(View::Duration(TimeView {
                unit: TimeUnit::Nanosecond,
                validity: get_bits_with_offset(array),
                values: array.values(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::DurationMicrosecondArray>() {
            Ok(View::Duration(TimeView {
                unit: TimeUnit::Microsecond,
                validity: get_bits_with_offset(array),
                values: array.values(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::DurationMillisecondArray>() {
            Ok(View::Duration(TimeView {
                unit: TimeUnit::Millisecond,
                validity: get_bits_with_offset(array),
                values: array.values(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::DurationSecondArray>() {
            Ok(View::Duration(TimeView {
                unit: TimeUnit::Second,
                validity: get_bits_with_offset(array),
                values: array.values(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::StringArray>() {
            Ok(View::Utf8(BytesView {
                validity: get_bits_with_offset(array),
                offsets: array.value_offsets(),
                data: array.value_data(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::LargeStringArray>() {
            Ok(View::LargeUtf8(BytesView {
                validity: get_bits_with_offset(array),
                offsets: array.value_offsets(),
                data: array.value_data(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::BinaryArray>() {
            Ok(View::Binary(BytesView {
                validity: get_bits_with_offset(array),
                offsets: array.value_offsets(),
                data: array.value_data(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::LargeBinaryArray>() {
            Ok(View::LargeBinary(BytesView {
                validity: get_bits_with_offset(array),
                offsets: array.value_offsets(),
                data: array.value_data(),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::FixedSizeBinaryArray>() {
            wrap_fixed_size_binary_array(array)
        } else if let Some(array) = any.downcast_ref::<arrow_array::ListArray>() {
            use arrow_array::Array;

            let arrow_schema::DataType::List(field) = array.data_type() else {
                fail!(
                    ErrorKind::Unsupported,
                    "invalid data type for list array: {}",
                    array.data_type()
                );
            };
            Ok(View::List(ListView {
                validity: get_bits_with_offset(array),
                offsets: array.value_offsets(),
                meta: meta_from_field(field.as_ref().try_into()?),
                elements: Box::new(array.values().as_ref().try_into()?),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::LargeListArray>() {
            let arrow_schema::DataType::LargeList(field) = array.data_type() else {
                fail!(
                    ErrorKind::Unsupported,
                    "invalid data type for list array: {}",
                    array.data_type()
                );
            };
            Ok(View::LargeList(ListView {
                validity: get_bits_with_offset(array),
                offsets: array.value_offsets(),
                meta: meta_from_field(field.as_ref().try_into()?),
                elements: Box::new(array.values().as_ref().try_into()?),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::FixedSizeListArray>() {
            let arrow_schema::DataType::FixedSizeList(field, n) = array.data_type() else {
                fail!(
                    ErrorKind::Unsupported,
                    "invalid data type for list array: {}",
                    array.data_type()
                );
            };
            Ok(View::FixedSizeList(FixedSizeListView {
                len: array.len(),
                n: *n,
                validity: get_bits_with_offset(array),
                meta: meta_from_field(field.as_ref().try_into()?),
                elements: Box::new(array.values().as_ref().try_into()?),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::StructArray>() {
            let arrow_schema::DataType::Struct(column_fields) = array.data_type() else {
                fail!(
                    ErrorKind::Unsupported,
                    "invalid data type for struct array: {}",
                    array.data_type()
                );
            };

            let mut fields = Vec::new();
            for (field, array) in std::iter::zip(column_fields, array.columns()) {
                let view = View::try_from(array.as_ref())?;
                let meta = meta_from_field(Field::try_from(field.as_ref())?);
                fields.push((view, meta));
            }

            Ok(View::Struct(StructView {
                len: array.len(),
                validity: get_bits_with_offset(array),
                fields,
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::MapArray>() {
            let arrow_schema::DataType::Map(entries_field, _) = array.data_type() else {
                fail!(
                    ErrorKind::Unsupported,
                    "invalid data type for map array: {}",
                    array.data_type()
                );
            };
            let entries_array: &dyn arrow_array::Array = array.entries();

            Ok(View::Map(ListView {
                validity: get_bits_with_offset(array),
                offsets: array.value_offsets(),
                meta: meta_from_field(Field::try_from(entries_field.as_ref())?),
                elements: Box::new(entries_array.try_into()?),
            }))
        } else if let Some(array) = any.downcast_ref::<arrow_array::UInt8DictionaryArray>() {
            wrap_dictionary_array::<arrow_array::types::UInt8Type>(array)
        } else if let Some(array) = any.downcast_ref::<arrow_array::UInt16DictionaryArray>() {
            wrap_dictionary_array::<arrow_array::types::UInt16Type>(array)
        } else if let Some(array) = any.downcast_ref::<arrow_array::UInt32DictionaryArray>() {
            wrap_dictionary_array::<arrow_array::types::UInt32Type>(array)
        } else if let Some(array) = any.downcast_ref::<arrow_array::UInt64DictionaryArray>() {
            wrap_dictionary_array::<arrow_array::types::UInt64Type>(array)
        } else if let Some(array) = any.downcast_ref::<arrow_array::Int8DictionaryArray>() {
            wrap_dictionary_array::<arrow_array::types::Int8Type>(array)
        } else if let Some(array) = any.downcast_ref::<arrow_array::Int16DictionaryArray>() {
            wrap_dictionary_array::<arrow_array::types::Int16Type>(array)
        } else if let Some(array) = any.downcast_ref::<arrow_array::Int32DictionaryArray>() {
            wrap_dictionary_array::<arrow_array::types::Int32Type>(array)
        } else if let Some(array) = any.downcast_ref::<arrow_array::Int64DictionaryArray>() {
            wrap_dictionary_array::<arrow_array::types::Int64Type>(array)
        } else if let Some(array) = any.downcast_ref::<arrow_array::UnionArray>() {
            use arrow_array::Array;

            let arrow_schema::DataType::Union(union_fields, arrow_schema::UnionMode::Dense) =
                array.data_type()
            else {
                fail!(
                    ErrorKind::Unsupported,
                    "Invalid data type: only dense unions are supported"
                );
            };

            let mut fields = Vec::new();
            for (type_id, field) in union_fields.iter() {
                let meta = meta_from_field(Field::try_from(field.as_ref())?);
                let view: View = array.child(type_id).as_ref().try_into()?;
                fields.push((type_id, view, meta));
            }
            let Some(offsets) = array.offsets() else {
                fail!(
                    ErrorKind::Unsupported,
                    "Dense unions must have an offset array"
                );
            };

            Ok(View::DenseUnion(DenseUnionView {
                types: array.type_ids(),
                offsets,
                fields,
            }))
        } else {
            fail!(
                ErrorKind::Unsupported,
                "Cannot build an array view for {dt}",
                dt = array.data_type()
            );
        }
    }
}

fn field_from_data_and_meta(data: &arrow_data::ArrayData, meta: FieldMeta) -> arrow_schema::Field {
    arrow_schema::Field::new(meta.name, data.data_type().clone(), meta.nullable)
        .with_metadata(meta.metadata)
}

fn primitive_into_data<T: arrow_buffer::ArrowNativeType>(
    data_type: arrow_schema::DataType,
    validity: Option<Vec<u8>>,
    values: Vec<T>,
) -> Result<arrow_data::ArrayData> {
    Ok(arrow_data::ArrayData::try_new(
        data_type,
        values.len(),
        validity.map(arrow_buffer::Buffer::from_vec),
        0,
        vec![arrow_buffer::ScalarBuffer::from(values).into_inner()],
        vec![],
    )?)
}

fn bytes_into_data<O: arrow_buffer::ArrowNativeType>(
    data_type: arrow_schema::DataType,
    offsets: Vec<O>,
    data: Vec<u8>,
    validity: Option<Vec<u8>>,
) -> Result<arrow_data::ArrayData> {
    Ok(arrow_data::ArrayData::try_new(
        data_type,
        offsets.len().saturating_sub(1),
        validity.map(arrow_buffer::Buffer::from_vec),
        0,
        vec![
            arrow_buffer::ScalarBuffer::from(offsets).into_inner(),
            arrow_buffer::ScalarBuffer::from(data).into_inner(),
        ],
        vec![],
    )?)
}

fn list_into_data<O: arrow_buffer::ArrowNativeType>(
    data_type: arrow_schema::DataType,
    len: usize,
    offsets: Vec<O>,
    child_data: arrow_data::ArrayData,
    validity: Option<Vec<u8>>,
) -> Result<arrow_data::ArrayData> {
    Ok(arrow_data::ArrayData::try_new(
        data_type,
        len,
        validity.map(arrow_buffer::Buffer::from_vec),
        0,
        vec![arrow_buffer::ScalarBuffer::from(offsets).into_inner()],
        vec![child_data],
    )?)
}

fn wrap_dictionary_array<K: arrow_array::types::ArrowDictionaryKeyType>(
    array: &arrow_array::DictionaryArray<K>,
) -> Result<View<'_>> {
    let keys: &dyn arrow_array::Array = array.keys();

    Ok(View::Dictionary(DictionaryView {
        indices: Box::new(keys.try_into()?),
        values: Box::new(array.values().as_ref().try_into()?),
    }))
}

fn get_bits_with_offset(array: &dyn arrow_array::Array) -> Option<BitsWithOffset<'_>> {
    let validity = array.nulls()?;
    Some(BitsWithOffset {
        offset: validity.offset(),
        data: validity.validity(),
    })
}
