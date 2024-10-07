use std::sync::Arc;

use super::arrow;

use crate::{
    array::{Array, BytesArray, PrimitiveArray},
    testing::{view_as, PanicOnError},
    view::{BitsWithOffset, BytesView, PrimitiveView, View},
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
            }
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
