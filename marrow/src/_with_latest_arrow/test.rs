use std::sync::Arc;

use super::arrow;

use crate::{
    array::{
        Array, BooleanArray, BytesArray, FixedSizeBinaryArray, FixedSizeListArray, ListArray,
        NullArray, PrimitiveArray,
    },
    datatypes::FieldMeta,
    testing::{view_as, PanicOnError},
    view::{
        BitsWithOffset, BooleanView, BytesView, FixedSizeBinaryView, FixedSizeListView, ListView,
        NullView, PrimitiveView, View,
    },
};

fn as_array_ref<A: arrow::array::Array + 'static>(values: impl Into<A>) -> arrow::array::ArrayRef {
    Arc::new(values.into()) as arrow::array::ArrayRef
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
        let array_via_arrow = Arc::new(arrow::array::NullArray::new(3)) as ArrayRef;
        let array_via_marrow = ArrayRef::try_from(Array::Null(NullArray { len: 3 }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(view_as!(View::Null, array_via_arrow)?, NullView { len: 3 });

        Ok(())
    }
}

mod boolean {
    use super::*;

    use arrow::array::ArrayRef;

    #[test]
    fn non_nullable() -> PanicOnError<()> {
        let array_via_arrow =
            as_array_ref::<arrow::array::BooleanArray>(vec![true, true, false, false, false]);
        let array_via_marrow = ArrayRef::try_from(Array::Boolean(BooleanArray {
            len: 5,
            validity: None,
            values: vec![0b_00011],
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::Boolean, array_via_arrow)?,
            BooleanView {
                len: 5,
                validity: None,
                values: BitsWithOffset {
                    offset: 0,
                    data: &[0b_00011]
                },
            },
        );
        Ok(())
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        let array_via_arrow = as_array_ref::<arrow::array::BooleanArray>(vec![
            Some(true),
            None,
            None,
            Some(false),
            Some(false),
        ]);
        let array_via_marrow = ArrayRef::try_from(Array::Boolean(BooleanArray {
            len: 5,
            validity: Some(vec![0b_11001]),
            values: vec![0b_000001],
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::Boolean, array_via_arrow)?,
            BooleanView {
                len: 5,
                validity: Some(BitsWithOffset {
                    offset: 0,
                    data: &[0b_11001]
                }),
                values: BitsWithOffset {
                    offset: 0,
                    data: &[0b_00001]
                },
            },
        );
        Ok(())
    }
}

mod int8 {
    use super::*;

    use arrow::array::{ArrayRef, Int8Array};

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        let array_via_arrow = as_array_ref::<Int8Array>(vec![1, -2, 3, -4]);
        let array_via_marrow = ArrayRef::try_from(Array::Int8(PrimitiveArray {
            validity: None,
            values: vec![1, -2, 3, -4],
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::Int8, array_via_arrow)?,
            PrimitiveView {
                validity: None,
                values: &[1, -2, 3, -4],
            },
        );
        Ok(())
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        let array_via_arrow = as_array_ref::<Int8Array>(vec![Some(1), Some(-2), None, None]);
        let array_via_marrow = ArrayRef::try_from(Array::Int8(PrimitiveArray {
            validity: Some(vec![0b_0011]),
            values: vec![1, -2, 0, 0],
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::Int8, array_via_arrow)?,
            PrimitiveView {
                validity: Some(BitsWithOffset {
                    offset: 0,
                    data: &[0b_0011]
                }),
                values: &[1, -2, 0, 0],
            },
        );
        Ok(())
    }
}

mod int16 {
    use super::*;

    use arrow::array::{ArrayRef, Int16Array};

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        let array_via_arrow = as_array_ref::<Int16Array>(vec![1, -2, 3, -4]);
        let array_via_marrow = ArrayRef::try_from(Array::Int16(PrimitiveArray {
            validity: None,
            values: vec![1, -2, 3, -4],
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::Int16, array_via_arrow)?,
            PrimitiveView {
                validity: None,
                values: &[1, -2, 3, -4],
            }
        );
        Ok(())
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        let array_via_arrow = as_array_ref::<Int16Array>(vec![Some(1), Some(-2), None, None]);
        let array_via_marrow = ArrayRef::try_from(Array::Int16(PrimitiveArray {
            validity: Some(vec![0b_0011]),
            values: vec![1, -2, 0, 0],
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::Int16, array_via_arrow)?,
            PrimitiveView {
                validity: Some(BitsWithOffset {
                    offset: 0,
                    data: &[0b_0011]
                }),
                values: &[1, -2, 0, 0],
            },
        );
        Ok(())
    }
}

mod int32 {
    use super::*;

    use arrow::array::{ArrayRef, Int32Array};

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        let array_via_arrow = as_array_ref::<Int32Array>(vec![1, -2, 3, -4]);
        let array_via_marrow = ArrayRef::try_from(Array::Int32(PrimitiveArray {
            validity: None,
            values: vec![1, -2, 3, -4],
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::Int32, array_via_arrow)?,
            PrimitiveView {
                validity: None,
                values: &[1, -2, 3, -4],
            }
        );
        Ok(())
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        let array_via_arrow = as_array_ref::<Int32Array>(vec![Some(1), Some(-2), None, None]);
        let array_via_marrow = ArrayRef::try_from(Array::Int32(PrimitiveArray {
            validity: Some(vec![0b_0011]),
            values: vec![1, -2, 0, 0],
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::Int32, array_via_arrow)?,
            PrimitiveView {
                validity: Some(BitsWithOffset {
                    offset: 0,
                    data: &[0b_0011]
                }),
                values: &[1, -2, 0, 0],
            },
        );
        Ok(())
    }
}

mod int64 {
    use super::*;

    use arrow::array::{ArrayRef, Int64Array};

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        let array_via_arrow = as_array_ref::<Int64Array>(vec![1, -2, 3, -4]);
        let array_via_marrow = ArrayRef::try_from(Array::Int64(PrimitiveArray {
            validity: None,
            values: vec![1, -2, 3, -4],
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);

        assert_eq!(
            view_as!(View::Int64, array_via_arrow)?,
            PrimitiveView {
                validity: None,
                values: &[1, -2, 3, -4],
            }
        );

        let slice = array_via_arrow.slice(1, 3);
        assert_eq!(
            view_as!(View::Int64, slice)?,
            PrimitiveView {
                validity: None,
                values: &[-2, 3, -4],
            }
        );

        Ok(())
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        let array_via_arrow = as_array_ref::<Int64Array>(vec![Some(1), Some(-2), None, None]);
        let array_via_marrow = ArrayRef::try_from(Array::Int64(PrimitiveArray {
            validity: Some(vec![0b_0011]),
            values: vec![1, -2, 0, 0],
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);

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

        Ok(())
    }
}

mod uint8 {
    use super::*;

    use arrow::array::{ArrayRef, UInt8Array};

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        let array_via_arrow = as_array_ref::<UInt8Array>(vec![1, 2, 3, 4]);
        let array_via_marrow = ArrayRef::try_from(Array::UInt8(PrimitiveArray {
            validity: None,
            values: vec![1, 2, 3, 4],
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::UInt8, array_via_arrow)?,
            PrimitiveView {
                validity: None,
                values: &[1, 2, 3, 4],
            }
        );
        Ok(())
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        let array_via_arrow = as_array_ref::<UInt8Array>(vec![Some(1), Some(2), None, None]);
        let array_via_marrow = ArrayRef::try_from(Array::UInt8(PrimitiveArray {
            validity: Some(vec![0b_0011]),
            values: vec![1, 2, 0, 0],
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::UInt8, array_via_arrow)?,
            PrimitiveView {
                validity: Some(BitsWithOffset {
                    offset: 0,
                    data: &[0b_0011]
                }),
                values: &[1, 2, 0, 0],
            },
        );
        Ok(())
    }
}

mod uint16 {
    use super::*;

    use arrow::array::{ArrayRef, UInt16Array};

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        let array_via_arrow = as_array_ref::<UInt16Array>(vec![1, 2, 3, 4]);
        let array_via_marrow = ArrayRef::try_from(Array::UInt16(PrimitiveArray {
            validity: None,
            values: vec![1, 2, 3, 4],
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::UInt16, array_via_arrow)?,
            PrimitiveView {
                validity: None,
                values: &[1, 2, 3, 4],
            }
        );
        Ok(())
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        let array_via_arrow = as_array_ref::<UInt16Array>(vec![Some(1), Some(2), None, None]);
        let array_via_marrow = ArrayRef::try_from(Array::UInt16(PrimitiveArray {
            validity: Some(vec![0b_0011]),
            values: vec![1, 2, 0, 0],
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::UInt16, array_via_arrow)?,
            PrimitiveView {
                validity: Some(BitsWithOffset {
                    offset: 0,
                    data: &[0b_0011]
                }),
                values: &[1, 2, 0, 0],
            },
        );
        Ok(())
    }
}

mod uint32 {
    use super::*;

    use arrow::array::{ArrayRef, UInt32Array};

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        let array_via_arrow = as_array_ref::<UInt32Array>(vec![1, 2, 3, 4]);
        let array_via_marrow = ArrayRef::try_from(Array::UInt32(PrimitiveArray {
            validity: None,
            values: vec![1, 2, 3, 4],
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::UInt32, array_via_arrow)?,
            PrimitiveView {
                validity: None,
                values: &[1, 2, 3, 4],
            }
        );
        Ok(())
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        let array_via_arrow = as_array_ref::<UInt32Array>(vec![Some(1), Some(2), None, None]);
        let array_via_marrow = ArrayRef::try_from(Array::UInt32(PrimitiveArray {
            validity: Some(vec![0b_0011]),
            values: vec![1, 2, 0, 0],
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::UInt32, array_via_arrow)?,
            PrimitiveView {
                validity: Some(BitsWithOffset {
                    offset: 0,
                    data: &[0b_0011]
                }),
                values: &[1, 2, 0, 0],
            },
        );
        Ok(())
    }
}

mod uint64 {
    use super::*;

    use arrow::array::{ArrayRef, UInt64Array};

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        let array_via_arrow = as_array_ref::<UInt64Array>(vec![1, 2, 3, 4]);
        let array_via_marrow = ArrayRef::try_from(Array::UInt64(PrimitiveArray {
            validity: None,
            values: vec![1, 2, 3, 4],
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::UInt64, array_via_arrow)?,
            PrimitiveView {
                validity: None,
                values: &[1, 2, 3, 4],
            }
        );
        Ok(())
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        let array_via_arrow = as_array_ref::<UInt64Array>(vec![Some(1), Some(2), None, None]);
        let array_via_marrow = ArrayRef::try_from(Array::UInt64(PrimitiveArray {
            validity: Some(vec![0b_0011]),
            values: vec![1, 2, 0, 0],
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::UInt64, array_via_arrow)?,
            PrimitiveView {
                validity: Some(BitsWithOffset {
                    offset: 0,
                    data: &[0b_0011]
                }),
                values: &[1, 2, 0, 0],
            },
        );
        Ok(())
    }
}

mod float16 {
    use super::*;

    use arrow::array::{ArrayRef, Float16Array};
    use half::f16;

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        let array_via_arrow = as_array_ref::<Float16Array>(vec![
            f16::from_f64(13.0),
            f16::from_f64(21.0),
            f16::from_f64(42.0),
        ]);
        let array_via_marrow = ArrayRef::try_from(Array::Float16(PrimitiveArray {
            validity: None,
            values: vec![
                f16::from_f64(13.0),
                f16::from_f64(21.0),
                f16::from_f64(42.0),
            ],
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::Float16, array_via_arrow)?,
            PrimitiveView {
                validity: None,
                values: &[
                    f16::from_f64(13.0),
                    f16::from_f64(21.0),
                    f16::from_f64(42.0)
                ],
            }
        );
        Ok(())
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        let array_via_arrow =
            as_array_ref::<Float16Array>(vec![None, None, Some(f16::from_f64(42.0))]);
        let array_via_marrow = ArrayRef::try_from(Array::Float16(PrimitiveArray {
            validity: Some(vec![0b_100]),
            values: vec![f16::from_f64(0.0), f16::from_f64(0.0), f16::from_f64(42.0)],
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::Float16, array_via_arrow)?,
            PrimitiveView {
                validity: Some(BitsWithOffset {
                    offset: 0,
                    data: &[0b_100]
                }),
                values: &[f16::from_f64(0.0), f16::from_f64(0.0), f16::from_f64(42.0)],
            }
        );
        Ok(())
    }
}

mod float32 {
    use super::*;

    use arrow::array::{ArrayRef, Float32Array};

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        let array_via_arrow = as_array_ref::<Float32Array>(vec![13.0, 21.0, 42.0]);
        let array_via_marrow = ArrayRef::try_from(Array::Float32(PrimitiveArray {
            validity: None,
            values: vec![13.0, 21.0, 42.0],
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::Float32, array_via_arrow)?,
            PrimitiveView {
                validity: None,
                values: &[13.0, 21.0, 42.0],
            }
        );
        Ok(())
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        let array_via_arrow = as_array_ref::<Float32Array>(vec![None, None, Some(42.0)]);
        let array_via_marrow = ArrayRef::try_from(Array::Float32(PrimitiveArray {
            validity: Some(vec![0b_100]),
            values: vec![0.0, 0.0, 42.0],
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::Float32, array_via_arrow)?,
            PrimitiveView {
                validity: Some(BitsWithOffset {
                    offset: 0,
                    data: &[0b_100]
                }),
                values: &[0.0, 0.0, 42.0],
            }
        );
        Ok(())
    }
}

mod float64 {
    use super::*;

    use arrow::array::{ArrayRef, Float64Array};

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        let array_via_arrow = as_array_ref::<Float64Array>(vec![13.0, 21.0, 42.0]);
        let array_via_marrow = ArrayRef::try_from(Array::Float64(PrimitiveArray {
            validity: None,
            values: vec![13.0, 21.0, 42.0],
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::Float64, array_via_arrow)?,
            PrimitiveView {
                validity: None,
                values: &[13.0, 21.0, 42.0],
            }
        );
        Ok(())
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        let array_via_arrow = as_array_ref::<Float64Array>(vec![None, None, Some(42.0)]);
        let array_via_marrow = ArrayRef::try_from(Array::Float64(PrimitiveArray {
            validity: Some(vec![0b_100]),
            values: vec![0.0, 0.0, 42.0],
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::Float64, array_via_arrow)?,
            PrimitiveView {
                validity: Some(BitsWithOffset {
                    offset: 0,
                    data: &[0b_100]
                }),
                values: &[0.0, 0.0, 42.0],
            }
        );
        Ok(())
    }
}

mod utf8 {
    use super::*;

    use arrow::array::{ArrayRef, StringArray};

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        let array_via_arrow =
            as_array_ref::<StringArray>(vec!["foo", "bar", "baz", "hello", "world"]);
        let array_via_marrow = ArrayRef::try_from(Array::Utf8(BytesArray {
            validity: None,
            offsets: vec![0, 3, 6, 9, 14, 19],
            data: b"foobarbazhelloworld".to_vec(),
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::Utf8, array_via_arrow)?,
            BytesView {
                validity: None,
                offsets: &[0, 3, 6, 9, 14, 19],
                data: b"foobarbazhelloworld",
            },
        );
        Ok(())
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        let array_via_arrow =
            as_array_ref::<StringArray>(vec![Some("foo"), Some("bar"), None, None, Some("world")]);
        let array_via_marrow = ArrayRef::try_from(Array::Utf8(BytesArray {
            validity: Some(vec![0b_10011]),
            offsets: vec![0, 3, 6, 6, 6, 11],
            data: b"foobarworld".to_vec(),
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::Utf8, array_via_arrow)?,
            BytesView {
                validity: Some(BitsWithOffset {
                    offset: 0,
                    data: &[0b_10011],
                }),
                offsets: &[0, 3, 6, 6, 6, 11],
                data: b"foobarworld",
            },
        );
        Ok(())
    }
}

mod large_utf8 {
    use super::*;

    use arrow::array::{ArrayRef, LargeStringArray};

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        let array_via_arrow =
            as_array_ref::<LargeStringArray>(vec!["foo", "bar", "baz", "hello", "world"]);
        let array_via_marrow = ArrayRef::try_from(Array::LargeUtf8(BytesArray {
            validity: None,
            offsets: vec![0, 3, 6, 9, 14, 19],
            data: b"foobarbazhelloworld".to_vec(),
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::LargeUtf8, array_via_arrow)?,
            BytesView {
                validity: None,
                offsets: &[0, 3, 6, 9, 14, 19],
                data: b"foobarbazhelloworld",
            },
        );
        Ok(())
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        let array_via_arrow = as_array_ref::<LargeStringArray>(vec![
            Some("foo"),
            Some("bar"),
            None,
            None,
            Some("world"),
        ]);
        let array_via_marrow = ArrayRef::try_from(Array::LargeUtf8(BytesArray {
            validity: Some(vec![0b_10011]),
            offsets: vec![0, 3, 6, 6, 6, 11],
            data: b"foobarworld".to_vec(),
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::LargeUtf8, array_via_arrow)?,
            BytesView {
                validity: Some(BitsWithOffset {
                    offset: 0,
                    data: &[0b_10011],
                }),
                offsets: &[0, 3, 6, 6, 6, 11],
                data: b"foobarworld",
            },
        );
        Ok(())
    }
}

mod binary {
    use super::*;

    use arrow::array::ArrayRef;

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        let array_via_arrow = as_array_ref::<arrow::array::BinaryArray>(vec![
            b"foo" as &[u8],
            b"bar",
            b"baz",
            b"hello",
            b"world",
        ]);
        let array_via_marrow = ArrayRef::try_from(Array::Binary(BytesArray {
            validity: None,
            offsets: vec![0, 3, 6, 9, 14, 19],
            data: b"foobarbazhelloworld".to_vec(),
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::Binary, array_via_arrow)?,
            BytesView {
                validity: None,
                offsets: &[0, 3, 6, 9, 14, 19],
                data: b"foobarbazhelloworld",
            },
        );
        Ok(())
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        let array_via_arrow = as_array_ref::<arrow::array::BinaryArray>(vec![
            Some(b"foo" as &[u8]),
            Some(b"bar"),
            None,
            None,
            Some(b"world"),
        ]);
        let array_via_marrow = ArrayRef::try_from(Array::Binary(BytesArray {
            validity: Some(vec![0b_10011]),
            offsets: vec![0, 3, 6, 6, 6, 11],
            data: b"foobarworld".to_vec(),
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::Binary, array_via_arrow)?,
            BytesView {
                validity: Some(BitsWithOffset {
                    offset: 0,
                    data: &[0b_10011],
                }),
                offsets: &[0, 3, 6, 6, 6, 11],
                data: b"foobarworld",
            },
        );
        Ok(())
    }
}

mod large_binary {
    use super::*;

    use arrow::array::ArrayRef;

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        let array_via_arrow = as_array_ref::<arrow::array::LargeBinaryArray>(vec![
            b"foo" as &[u8],
            b"bar",
            b"baz",
            b"hello",
            b"world",
        ]);
        let array_via_marrow = ArrayRef::try_from(Array::LargeBinary(BytesArray {
            validity: None,
            offsets: vec![0, 3, 6, 9, 14, 19],
            data: b"foobarbazhelloworld".to_vec(),
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::LargeBinary, array_via_arrow)?,
            BytesView {
                validity: None,
                offsets: &[0, 3, 6, 9, 14, 19],
                data: b"foobarbazhelloworld",
            },
        );
        Ok(())
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        let array_via_arrow = as_array_ref::<arrow::array::LargeBinaryArray>(vec![
            Some(b"foo" as &[u8]),
            Some(b"bar"),
            None,
            None,
            Some(b"world"),
        ]);
        let array_via_marrow = ArrayRef::try_from(Array::LargeBinary(BytesArray {
            validity: Some(vec![0b_10011]),
            offsets: vec![0, 3, 6, 6, 6, 11],
            data: b"foobarworld".to_vec(),
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::LargeBinary, array_via_arrow)?,
            BytesView {
                validity: Some(BitsWithOffset {
                    offset: 0,
                    data: &[0b_10011],
                }),
                offsets: &[0, 3, 6, 6, 6, 11],
                data: b"foobarworld",
            },
        );
        Ok(())
    }
}

mod fixed_size_binary {
    use super::*;

    use arrow::array::ArrayRef;

    #[test]
    fn not_nullable() -> PanicOnError<()> {
        let array_via_arrow = as_array_ref::<arrow::array::FixedSizeBinaryArray>(vec![
            b"foo" as &[u8],
            b"bar",
            b"baz",
        ]);
        let array_via_marrow = ArrayRef::try_from(Array::FixedSizeBinary(FixedSizeBinaryArray {
            validity: None,
            n: 3,
            data: b"foobarbaz".to_vec(),
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::FixedSizeBinary, array_via_arrow)?,
            FixedSizeBinaryView {
                validity: None,
                n: 3,
                data: b"foobarbaz",
            },
        );
        Ok(())
    }

    #[test]
    fn nullable() -> PanicOnError<()> {
        let array_via_arrow = as_array_ref::<arrow::array::FixedSizeBinaryArray>(vec![
            Some(b"foo" as &[u8]),
            Some(b"bar"),
            None,
            None,
        ]);
        let array_via_marrow = ArrayRef::try_from(Array::FixedSizeBinary(FixedSizeBinaryArray {
            validity: Some(vec![0b_0011]),
            n: 3,
            data: b"foobar\0\0\0\0\0\0".to_vec(),
        }))?;

        assert_eq!(&array_via_arrow, &array_via_marrow);
        assert_eq!(
            view_as!(View::FixedSizeBinary, array_via_arrow)?,
            FixedSizeBinaryView {
                validity: Some(BitsWithOffset {
                    offset: 0,
                    data: &[0b_0011],
                }),
                n: 3,
                data: b"foobar\0\0\0\0\0\0",
            },
        );
        Ok(())
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
        let array_via_arrow = example();
        let array_via_marrow = ArrayRef::try_from(Array::List(ListArray {
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
        }))?;

        assert_eq!(array_via_arrow.data_type(), array_via_marrow.data_type());
        assert_eq!(&array_via_arrow, &array_via_marrow);

        assert_eq!(
            view_as!(View::List, array_via_arrow)?,
            ListView {
                validity: Some(BitsWithOffset {
                    offset: 0,
                    data: &[0b_1101]
                }),
                offsets: &[0, 3, 3, 3, 4],
                meta: FieldMeta {
                    name: String::from("item"),
                    nullable: true,
                    metadata: Default::default()
                },
                elements: Box::new(View::Int32(PrimitiveView {
                    validity: Some(BitsWithOffset {
                        offset: 0,
                        data: &[0b_0111]
                    }),
                    values: &[1, 2, 3, 0]
                })),
            }
        );
        Ok(())
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
        let array_via_arrow = example();
        let array_via_marrow = ArrayRef::try_from(Array::LargeList(ListArray {
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
        }))?;

        assert_eq!(array_via_arrow.data_type(), array_via_marrow.data_type());
        assert_eq!(&array_via_arrow, &array_via_marrow);

        assert_eq!(
            view_as!(View::LargeList, array_via_arrow)?,
            ListView {
                validity: Some(BitsWithOffset {
                    offset: 0,
                    data: &[0b_1101]
                }),
                offsets: &[0, 3, 3, 3, 4],
                meta: FieldMeta {
                    name: String::from("item"),
                    nullable: true,
                    metadata: Default::default()
                },
                elements: Box::new(View::Int32(PrimitiveView {
                    validity: Some(BitsWithOffset {
                        offset: 0,
                        data: &[0b_0111]
                    }),
                    values: &[1, 2, 3, 0]
                })),
            }
        );
        Ok(())
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
        let array_via_arrow = example();
        let array_via_marrow = ArrayRef::try_from(Array::FixedSizeList(FixedSizeListArray {
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
        }))?;

        assert_eq!(array_via_arrow.data_type(), array_via_marrow.data_type());
        assert_eq!(&array_via_arrow, &array_via_marrow);

        assert_eq!(
            view_as!(View::FixedSizeList, array_via_arrow)?,
            FixedSizeListView {
                len: 4,
                n: 3,
                validity: Some(BitsWithOffset {
                    offset: 0,
                    data: &[0b_1101]
                }),
                meta: FieldMeta {
                    name: String::from("item"),
                    nullable: true,
                    metadata: Default::default()
                },
                elements: Box::new(View::Int32(PrimitiveView {
                    validity: Some(BitsWithOffset {
                        offset: 0,
                        data: &[0b_01_000_111, 0b_011_1],
                    }),
                    values: &[0, 1, 2, 0, 0, 0, 3, 0, 5, 6, 7, 0]
                })),
            }
        );
        Ok(())
    }
}
