use std::sync::Arc;

use super::super::arrow;

use crate::{
    array::{
        Array, BooleanArray, BytesArray, DenseUnionArray, FixedSizeBinaryArray, FixedSizeListArray,
        ListArray, NullArray, PrimitiveArray,
    },
    datatypes::FieldMeta,
    testing::{view_as, PanicOnError},
    view::{BitsWithOffset, PrimitiveView, View},
};

fn as_array_ref<A: arrow::array::Array + 'static>(values: impl Into<A>) -> arrow::array::ArrayRef {
    Arc::new(values.into()) as arrow::array::ArrayRef
}

fn assert_arrays_eq(
    array_via_arrow: arrow::array::ArrayRef,
    marrow_array: Array,
) -> PanicOnError<()> {
    let array_via_marrow = arrow::array::ArrayRef::try_from(marrow_array.clone())?;
    assert_eq!(array_via_arrow.data_type(), array_via_marrow.data_type());
    assert_eq!(&array_via_arrow, &array_via_marrow);

    let view_via_arrow = View::try_from(&*array_via_arrow)?;
    let view_via_marrow = marrow_array.as_view();
    assert_eq!(view_via_arrow, view_via_marrow);

    Ok(())
}

#[test]
fn slicing() -> PanicOnError<()> {
    let array_via_arrow =
        as_array_ref::<arrow::array::Int64Array>(vec![Some(1), Some(-2), None, None]);

    assert_eq!(
        view_as!(View::Int64, array_via_arrow)?,
        PrimitiveView {
            validity: Some(BitsWithOffset {
                offset: 0,
                data: &[0b_0011]
            }),
            values: &[1, -2, 0, 0],
        },
    );

    let slice_ref = array_via_arrow.slice(1, 3);
    assert_eq!(
        view_as!(View::Int64, slice_ref)?,
        PrimitiveView {
            validity: Some(BitsWithOffset {
                offset: 1,
                data: &[0b_0011]
            }),
            values: &[-2, 0, 0],
        },
    );

    let slice_ref = array_via_arrow.slice(2, 2);
    assert_eq!(
        view_as!(View::Int64, slice_ref)?,
        PrimitiveView {
            validity: Some(BitsWithOffset {
                offset: 2,
                data: &[0b_0011]
            }),
            values: &[0, 0],
        },
    );

    let slice_ref = array_via_arrow.slice(3, 1);
    assert_eq!(
        view_as!(View::Int64, slice_ref)?,
        PrimitiveView {
            validity: Some(BitsWithOffset {
                offset: 3,
                data: &[0b_0011]
            }),
            values: &[0],
        },
    );

    let slice_ref = array_via_arrow.slice(4, 0);
    assert_eq!(
        view_as!(View::Int64, slice_ref)?,
        PrimitiveView {
            validity: Some(BitsWithOffset {
                offset: 4,
                data: &[0b_0011]
            }),
            values: &[],
        },
    );

    Ok(())
}

mod null {
    use super::*;

    use arrow::array::ArrayRef;

    #[test]
    fn example() -> PanicOnError<()> {
        assert_arrays_eq(
            Arc::new(arrow::array::NullArray::new(3)) as ArrayRef,
            Array::Null(NullArray { len: 3 }),
        )
    }
}

mod boolean {
    use super::*;

    #[test]
    fn non_nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::BooleanArray>(vec![true, true, false, false, false]),
            Array::Boolean(BooleanArray {
                len: 5,
                validity: None,
                values: vec![0b_00011],
            }),
        )
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::BooleanArray>(vec![
                Some(true),
                None,
                None,
                Some(false),
                Some(false),
            ]),
            Array::Boolean(BooleanArray {
                len: 5,
                validity: Some(vec![0b_11001]),
                values: vec![0b_000001],
            }),
        )
    }
}

mod int8 {
    use super::*;

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::Int8Array>(vec![1, -2, 3, -4]),
            Array::Int8(PrimitiveArray {
                validity: None,
                values: vec![1, -2, 3, -4],
            }),
        )
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::Int8Array>(vec![Some(1), Some(-2), None, None]),
            Array::Int8(PrimitiveArray {
                validity: Some(vec![0b_0011]),
                values: vec![1, -2, 0, 0],
            }),
        )
    }
}

mod int16 {
    use super::*;

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::Int16Array>(vec![1, -2, 3, -4]),
            Array::Int16(PrimitiveArray {
                validity: None,
                values: vec![1, -2, 3, -4],
            }),
        )
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::Int16Array>(vec![Some(1), Some(-2), None, None]),
            Array::Int16(PrimitiveArray {
                validity: Some(vec![0b_0011]),
                values: vec![1, -2, 0, 0],
            }),
        )
    }
}

mod int32 {
    use super::*;

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::Int32Array>(vec![1, -2, 3, -4]),
            Array::Int32(PrimitiveArray {
                validity: None,
                values: vec![1, -2, 3, -4],
            }),
        )
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::Int32Array>(vec![Some(1), Some(-2), None, None]),
            Array::Int32(PrimitiveArray {
                validity: Some(vec![0b_0011]),
                values: vec![1, -2, 0, 0],
            }),
        )
    }
}

mod int64 {
    use super::*;

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::Int64Array>(vec![1, -2, 3, -4]),
            Array::Int64(PrimitiveArray {
                validity: None,
                values: vec![1, -2, 3, -4],
            }),
        )
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::Int64Array>(vec![Some(1), Some(-2), None, None]),
            Array::Int64(PrimitiveArray {
                validity: Some(vec![0b_0011]),
                values: vec![1, -2, 0, 0],
            }),
        )
    }
}

mod uint8 {
    use super::*;

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::UInt8Array>(vec![1, 2, 3, 4]),
            Array::UInt8(PrimitiveArray {
                validity: None,
                values: vec![1, 2, 3, 4],
            }),
        )
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::UInt8Array>(vec![Some(1), Some(2), None, None]),
            Array::UInt8(PrimitiveArray {
                validity: Some(vec![0b_0011]),
                values: vec![1, 2, 0, 0],
            }),
        )
    }
}

mod uint16 {
    use super::*;

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::UInt16Array>(vec![1, 2, 3, 4]),
            Array::UInt16(PrimitiveArray {
                validity: None,
                values: vec![1, 2, 3, 4],
            }),
        )
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::UInt16Array>(vec![Some(1), Some(2), None, None]),
            Array::UInt16(PrimitiveArray {
                validity: Some(vec![0b_0011]),
                values: vec![1, 2, 0, 0],
            }),
        )
    }
}

mod uint32 {
    use super::*;

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::UInt32Array>(vec![1, 2, 3, 4]),
            Array::UInt32(PrimitiveArray {
                validity: None,
                values: vec![1, 2, 3, 4],
            }),
        )
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::UInt32Array>(vec![Some(1), Some(2), None, None]),
            Array::UInt32(PrimitiveArray {
                validity: Some(vec![0b_0011]),
                values: vec![1, 2, 0, 0],
            }),
        )
    }
}

mod uint64 {
    use super::*;

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::UInt64Array>(vec![1, 2, 3, 4]),
            Array::UInt64(PrimitiveArray {
                validity: None,
                values: vec![1, 2, 3, 4],
            }),
        )
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::UInt64Array>(vec![Some(1), Some(2), None, None]),
            Array::UInt64(PrimitiveArray {
                validity: Some(vec![0b_0011]),
                values: vec![1, 2, 0, 0],
            }),
        )
    }
}

mod float16 {
    use super::*;

    use half::f16;

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::Float16Array>(vec![
                f16::from_f64(13.0),
                f16::from_f64(21.0),
                f16::from_f64(42.0),
            ]),
            Array::Float16(PrimitiveArray {
                validity: None,
                values: vec![
                    f16::from_f64(13.0),
                    f16::from_f64(21.0),
                    f16::from_f64(42.0),
                ],
            }),
        )
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::Float16Array>(vec![None, None, Some(f16::from_f64(42.0))]),
            Array::Float16(PrimitiveArray {
                validity: Some(vec![0b_100]),
                values: vec![f16::from_f64(0.0), f16::from_f64(0.0), f16::from_f64(42.0)],
            }),
        )
    }
}

mod float32 {
    use super::*;

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::Float32Array>(vec![13.0, 21.0, 42.0]),
            Array::Float32(PrimitiveArray {
                validity: None,
                values: vec![13.0, 21.0, 42.0],
            }),
        )
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::Float32Array>(vec![None, None, Some(42.0)]),
            Array::Float32(PrimitiveArray {
                validity: Some(vec![0b_100]),
                values: vec![0.0, 0.0, 42.0],
            }),
        )
    }
}

mod float64 {
    use super::*;

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::Float64Array>(vec![13.0, 21.0, 42.0]),
            Array::Float64(PrimitiveArray {
                validity: None,
                values: vec![13.0, 21.0, 42.0],
            }),
        )
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::Float64Array>(vec![None, None, Some(42.0)]),
            Array::Float64(PrimitiveArray {
                validity: Some(vec![0b_100]),
                values: vec![0.0, 0.0, 42.0],
            }),
        )
    }
}

mod date32 {
    use super::*;

    use arrow::datatypes::Date32Type;
    use chrono::NaiveDate;

    fn ymd_as_num(y: i32, m: u32, d: u32) -> i32 {
        let unix_epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
        (NaiveDate::from_ymd_opt(y, m, d).unwrap() - unix_epoch).num_days() as i32
    }

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::Date32Array>(vec![
                Date32Type::from_naive_date(NaiveDate::from_ymd_opt(2024, 10, 8).unwrap()),
                Date32Type::from_naive_date(NaiveDate::from_ymd_opt(-10, 12, 31).unwrap()),
            ]),
            Array::Date32(PrimitiveArray {
                validity: None,
                values: vec![ymd_as_num(2024, 10, 8), ymd_as_num(-10, 12, 31)],
            }),
        )
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::Date32Array>(vec![
                Some(Date32Type::from_naive_date(
                    NaiveDate::from_ymd_opt(2024, 10, 8).unwrap(),
                )),
                None,
                Some(Date32Type::from_naive_date(
                    NaiveDate::from_ymd_opt(-10, 12, 31).unwrap(),
                )),
            ]),
            Array::Date32(PrimitiveArray {
                validity: Some(vec![0b_101]),
                values: vec![ymd_as_num(2024, 10, 8), 0, ymd_as_num(-10, 12, 31)],
            }),
        )
    }
}

mod date64 {
    use super::*;

    use arrow::datatypes::Date64Type;
    use chrono::NaiveDate;

    fn ymd_as_num(y: i32, m: u32, d: u32) -> i64 {
        let unix_epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
        (NaiveDate::from_ymd_opt(y, m, d).unwrap() - unix_epoch).num_days() as i64
            * (24 * 60 * 60 * 1000)
    }

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::Date64Array>(vec![
                Date64Type::from_naive_date(NaiveDate::from_ymd_opt(2024, 10, 8).unwrap()),
                Date64Type::from_naive_date(NaiveDate::from_ymd_opt(-10, 12, 31).unwrap()),
            ]),
            Array::Date64(PrimitiveArray {
                validity: None,
                values: vec![ymd_as_num(2024, 10, 8), ymd_as_num(-10, 12, 31)],
            }),
        )
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::Date64Array>(vec![
                Some(Date64Type::from_naive_date(
                    NaiveDate::from_ymd_opt(2024, 10, 8).unwrap(),
                )),
                None,
                Some(Date64Type::from_naive_date(
                    NaiveDate::from_ymd_opt(-10, 12, 31).unwrap(),
                )),
            ]),
            Array::Date64(PrimitiveArray {
                validity: Some(vec![0b_101]),
                values: vec![ymd_as_num(2024, 10, 8), 0, ymd_as_num(-10, 12, 31)],
            }),
        )
    }
}

mod utf8 {
    use super::*;

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::StringArray>(vec!["foo", "bar", "baz", "hello", "world"]),
            Array::Utf8(BytesArray {
                validity: None,
                offsets: vec![0, 3, 6, 9, 14, 19],
                data: b"foobarbazhelloworld".to_vec(),
            }),
        )
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::StringArray>(vec![
                Some("foo"),
                Some("bar"),
                None,
                None,
                Some("world"),
            ]),
            Array::Utf8(BytesArray {
                validity: Some(vec![0b_10011]),
                offsets: vec![0, 3, 6, 6, 6, 11],
                data: b"foobarworld".to_vec(),
            }),
        )
    }
}

mod large_utf8 {
    use super::*;

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::LargeStringArray>(vec![
                "foo", "bar", "baz", "hello", "world",
            ]),
            Array::LargeUtf8(BytesArray {
                validity: None,
                offsets: vec![0, 3, 6, 9, 14, 19],
                data: b"foobarbazhelloworld".to_vec(),
            }),
        )
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::LargeStringArray>(vec![
                Some("foo"),
                Some("bar"),
                None,
                None,
                Some("world"),
            ]),
            Array::LargeUtf8(BytesArray {
                validity: Some(vec![0b_10011]),
                offsets: vec![0, 3, 6, 6, 6, 11],
                data: b"foobarworld".to_vec(),
            }),
        )
    }
}

mod binary {
    use super::*;

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::BinaryArray>(vec![
                b"foo" as &[u8],
                b"bar",
                b"baz",
                b"hello",
                b"world",
            ]),
            Array::Binary(BytesArray {
                validity: None,
                offsets: vec![0, 3, 6, 9, 14, 19],
                data: b"foobarbazhelloworld".to_vec(),
            }),
        )
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::BinaryArray>(vec![
                Some(b"foo" as &[u8]),
                Some(b"bar"),
                None,
                None,
                Some(b"world"),
            ]),
            Array::Binary(BytesArray {
                validity: Some(vec![0b_10011]),
                offsets: vec![0, 3, 6, 6, 6, 11],
                data: b"foobarworld".to_vec(),
            }),
        )
    }
}

mod large_binary {
    use super::*;

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::LargeBinaryArray>(vec![
                b"foo" as &[u8],
                b"bar",
                b"baz",
                b"hello",
                b"world",
            ]),
            Array::LargeBinary(BytesArray {
                validity: None,
                offsets: vec![0, 3, 6, 9, 14, 19],
                data: b"foobarbazhelloworld".to_vec(),
            }),
        )
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::LargeBinaryArray>(vec![
                Some(b"foo" as &[u8]),
                Some(b"bar"),
                None,
                None,
                Some(b"world"),
            ]),
            Array::LargeBinary(BytesArray {
                validity: Some(vec![0b_10011]),
                offsets: vec![0, 3, 6, 6, 6, 11],
                data: b"foobarworld".to_vec(),
            }),
        )
    }
}

mod fixed_size_binary {
    use super::*;

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::FixedSizeBinaryArray>(vec![
                b"foo" as &[u8],
                b"bar",
                b"baz",
            ]),
            Array::FixedSizeBinary(FixedSizeBinaryArray {
                validity: None,
                n: 3,
                data: b"foobarbaz".to_vec(),
            }),
        )
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            as_array_ref::<arrow::array::FixedSizeBinaryArray>(vec![
                Some(b"foo" as &[u8]),
                Some(b"bar"),
                None,
                None,
            ]),
            Array::FixedSizeBinary(FixedSizeBinaryArray {
                validity: Some(vec![0b_0011]),
                n: 3,
                data: b"foobar\0\0\0\0\0\0".to_vec(),
            }),
        )
    }
}

mod list {
    use super::*;

    use arrow::array::{ArrayRef, Int32Builder, ListBuilder};

    // example from the arrow docs
    fn example() -> ArrayRef {
        let mut builder = ListBuilder::new(Int32Builder::new());

        builder.append_value([Some(1), Some(2), Some(3)]);
        builder.append_null();
        builder.append_value([]);
        builder.append_value([None]);

        Arc::new(builder.finish()) as ArrayRef
    }

    #[test]
    fn non_nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            example(),
            Array::List(ListArray {
                validity: Some(vec![0b_1101]),
                offsets: vec![0, 3, 3, 3, 4],
                meta: FieldMeta {
                    name: String::from("item"),
                    nullable: true,
                    metadata: Default::default(),
                },
                elements: Box::new(Array::Int32(PrimitiveArray {
                    validity: Some(vec![0b_0111]),
                    values: vec![1, 2, 3, 0],
                })),
            }),
        )
    }
}

mod large_list {
    use super::*;

    use arrow::array::{ArrayRef, Int32Builder, LargeListBuilder};

    /// example from the arrow docs
    fn example() -> ArrayRef {
        let mut builder = LargeListBuilder::new(Int32Builder::new());

        builder.append_value([Some(1), Some(2), Some(3)]);
        builder.append_null();
        builder.append_value([]);
        builder.append_value([None]);

        Arc::new(builder.finish()) as ArrayRef
    }

    #[test]
    fn non_nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            example(),
            Array::LargeList(ListArray {
                validity: Some(vec![0b_1101]),
                offsets: vec![0, 3, 3, 3, 4],
                meta: FieldMeta {
                    name: String::from("item"),
                    nullable: true,
                    metadata: Default::default(),
                },
                elements: Box::new(Array::Int32(PrimitiveArray {
                    validity: Some(vec![0b_0111]),
                    values: vec![1, 2, 3, 0],
                })),
            }),
        )
    }
}

mod fixed_size_list {
    use super::*;

    use arrow::array::{ArrayRef, FixedSizeListBuilder, Int32Builder};

    // example from the arrow docs
    fn example() -> ArrayRef {
        let mut builder = FixedSizeListBuilder::new(Int32Builder::new(), 3);

        //  [[0, 1, 2], null, [3, null, 5], [6, 7, null]]
        builder.values().append_value(0);
        builder.values().append_value(1);
        builder.values().append_value(2);
        builder.append(true);
        builder.values().append_null();
        builder.values().append_null();
        builder.values().append_null();
        builder.append(false);
        builder.values().append_value(3);
        builder.values().append_null();
        builder.values().append_value(5);
        builder.append(true);
        builder.values().append_value(6);
        builder.values().append_value(7);
        builder.values().append_null();
        builder.append(true);

        Arc::new(builder.finish()) as ArrayRef
    }

    #[test]
    fn non_nullable() -> PanicOnError<()> {
        assert_arrays_eq(
            example(),
            Array::FixedSizeList(FixedSizeListArray {
                len: 4,
                n: 3,
                validity: Some(vec![0b_1101]),
                meta: FieldMeta {
                    name: String::from("item"),
                    nullable: true,
                    metadata: Default::default(),
                },
                elements: Box::new(Array::Int32(PrimitiveArray {
                    validity: Some(vec![0b_01_000_111, 0b_011_1]),
                    values: vec![0, 1, 2, 0, 0, 0, 3, 0, 5, 6, 7, 0],
                })),
            }),
        )
    }
}

mod dense_union_array {
    use super::*;

    use arrow::array::{ArrayRef, Float64Array, Int32Array, UnionArray};

    // example from arrow docs
    fn example_array() -> PanicOnError<ArrayRef> {
        let int_array = Int32Array::from(vec![1, 34]);
        let float_array = Float64Array::from(vec![3.2]);
        let type_ids = vec![0_i8, 1, 0];
        let offsets = vec![0, 0, 1];

        let union_fields = vec![
            (
                0_i8,
                Arc::new(arrow::datatypes::Field::new(
                    "A",
                    arrow::datatypes::DataType::Int32,
                    false,
                )),
            ),
            (
                1_i8,
                Arc::new(arrow::datatypes::Field::new(
                    "B",
                    arrow::datatypes::DataType::Float64,
                    false,
                )),
            ),
        ];

        let children = vec![Arc::new(int_array) as ArrayRef, Arc::new(float_array)];

        let array = UnionArray::try_new(
            union_fields.into_iter().collect(),
            type_ids.into(),
            Some(offsets.into()),
            children,
        )?;

        Ok(Arc::new(array) as ArrayRef)
    }

    #[test]
    fn example() -> PanicOnError<()> {
        assert_arrays_eq(
            example_array()?,
            Array::DenseUnion(DenseUnionArray {
                types: vec![0, 1, 0],
                offsets: vec![0, 0, 1],
                fields: vec![
                    (
                        0,
                        FieldMeta {
                            name: String::from("A"),
                            ..Default::default()
                        },
                        Array::Int32(PrimitiveArray {
                            validity: None,
                            values: vec![1, 34],
                        }),
                    ),
                    (
                        1,
                        FieldMeta {
                            name: String::from("B"),
                            ..Default::default()
                        },
                        Array::Float64(PrimitiveArray {
                            validity: None,
                            values: vec![3.2],
                        }),
                    ),
                ],
            }),
        )
    }
}
