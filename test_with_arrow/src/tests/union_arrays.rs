use std::sync::Arc;

use marrow::{
    array::{Array, DenseUnionArray, PrimitiveArray},
    datatypes::FieldMeta,
};

use super::utils::{assert_arrays_eq, PanicOnError};

mod dense_union_array {
    use super::*;

    use arrow_array::{ArrayRef, Float64Array, Int32Array, UnionArray};

    // Adapted from the arrow docs
    //
    // License: Apache Software License 2.0
    // Source: https://github.com/apache/arrow-rs/blob/065c7b8f94264eeb6a1ca23a92795fc4e0d31d51/arrow-array/src/array/union_array.rs#L48
    fn example_array() -> PanicOnError<ArrayRef> {
        let int_array = Int32Array::from(vec![1, 34]);
        let float_array = Float64Array::from(vec![3.2]);
        let type_ids = vec![0_i8, 1, 0];
        let offsets = vec![0, 0, 1];

        let union_fields = vec![
            (
                0_i8,
                Arc::new(arrow_schema::Field::new(
                    "A",
                    arrow_schema::DataType::Int32,
                    false,
                )),
            ),
            (
                1_i8,
                Arc::new(arrow_schema::Field::new(
                    "B",
                    arrow_schema::DataType::Float64,
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
