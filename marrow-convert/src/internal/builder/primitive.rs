use marrow::{
    array::{Array, BooleanArray, NullArray, PrimitiveArray},
    types::f16,
};

use crate::Result;

use super::{ArrayBuilder, ArrayPush, DefaultArrayBuilder};

#[derive(Debug, Default)]
struct PrimitiveBuilder<T, B> {
    values: Vec<T>,
    build_impl: B,
}

trait BuildPrimitiveArrayImpl<T> {
    fn build(&self, values: &mut Vec<T>) -> Result<Array>;
}

impl<T: Default, B: BuildPrimitiveArrayImpl<T>> ArrayBuilder for PrimitiveBuilder<T, B> {
    fn push_default(&mut self) -> Result<()> {
        self.values.push(T::default());
        Ok(())
    }

    fn build_array(&mut self) -> Result<Array> {
        self.build_impl.build(&mut self.values)
    }
}

#[derive(Debug, Default)]
struct BuildNative;

macro_rules! impl_build_native {
    ($(($ty:ident, $variant:ident),)*) => {
        $(
            impl BuildPrimitiveArrayImpl<$ty> for BuildNative {
                fn build(&self, values: &mut Vec<$ty>) -> Result<Array> {
                    Ok(Array::$variant(PrimitiveArray {
                        validity: None,
                        values: std::mem::take(values),
                    }))
                }
            }
        )*
    };
}

impl_build_native!(
    (i8, Int8),
    (i16, Int16),
    (i32, Int32),
    (i64, Int64),
    (u8, UInt8),
    (u16, UInt16),
    (u32, UInt32),
    (u64, UInt64),
    (f16, Float16),
    (f32, Float32),
    (f64, Float64),
);

macro_rules! define_builder {
    ($(($builder:ident, $ty:ident),)*) => {
        $(
            #[derive(Debug, Default)]
            pub struct $builder(PrimitiveBuilder<$ty, BuildNative>);

            impl ArrayBuilder for $builder {
                fn push_default(&mut self) -> Result<()> {
                    self.0.push_default()
                }

                fn build_array(&mut self) -> Result<Array> {
                    self.0.build_array()
                }
            }

            impl ArrayPush<$ty> for $builder {
                fn push_value(&mut self, value: &$ty) -> Result<()> {
                    self.0.values.push(*value);
                    Ok(())
                }
            }

            impl DefaultArrayBuilder for $ty {
                type ArrayBuilder = $builder;

                fn default_builder() -> Self::ArrayBuilder {
                    $builder::default()
                }
            }
        )*
    };
}

define_builder!(
    (Int8Builder, i8),
    (Int16Builder, i16),
    (Int32Builder, i32),
    (Int64Builder, i64),
    (UInt8Builder, u8),
    (UInt16Builder, u16),
    (UInt32Builder, u32),
    (UInt64Builder, u64),
    (Float16Builder, f16),
    (Float32Builder, f32),
    (Float64Builder, f64),
);

#[derive(Debug, Default)]
pub struct NullBuilder(usize);

impl ArrayBuilder for NullBuilder {
    fn push_default(&mut self) -> Result<()> {
        self.0 += 1;
        Ok(())
    }

    fn build_array(&mut self) -> Result<Array> {
        Ok(Array::Null(NullArray {
            len: std::mem::take(&mut self.0),
        }))
    }
}

impl DefaultArrayBuilder for () {
    type ArrayBuilder = NullBuilder;

    fn default_builder() -> Self::ArrayBuilder {
        NullBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct BooleanBuilder {
    len: usize,
    values: Vec<u8>,
}

impl ArrayBuilder for BooleanBuilder {
    fn push_default(&mut self) -> Result<()> {
        marrow::bits::push(&mut self.values, &mut self.len, false);
        Ok(())
    }

    fn build_array(&mut self) -> Result<Array> {
        Ok(Array::Boolean(BooleanArray {
            len: std::mem::take(&mut self.len),
            values: std::mem::take(&mut self.values),
            validity: None,
        }))
    }
}

impl DefaultArrayBuilder for bool {
    type ArrayBuilder = BooleanBuilder;

    fn default_builder() -> Self::ArrayBuilder {
        BooleanBuilder::default()
    }
}
