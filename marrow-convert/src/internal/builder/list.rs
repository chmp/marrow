use marrow::{
    array::{Array, ListArray},
    datatypes::FieldMeta,
};

use crate::{Error, Result};

use super::{ArrayBuilder, ArrayPush, DefaultArrayBuilder};

struct GenericListBuilder<O, B> {
    offsets: Vec<O>,
    builder: B,
}

impl<O: Offset, B> GenericListBuilder<O, B> {
    pub fn new(builder: B) -> Self {
        Self {
            offsets: vec![O::default()],
            builder,
        }
    }
}

trait Offset: Default + Copy + std::ops::Add<Self, Output = Self> {
    const ONE: Self;
    const ARRAY_VARIANT: fn(ListArray<Self>) -> Array;
}

impl Offset for i32 {
    const ONE: Self = 1;
    const ARRAY_VARIANT: fn(ListArray<Self>) -> Array = Array::List;
}

impl Offset for i64 {
    const ONE: Self = 1;
    const ARRAY_VARIANT: fn(ListArray<Self>) -> Array = Array::LargeList;
}

impl<O: Offset, B: ArrayBuilder> ArrayBuilder for GenericListBuilder<O, B> {
    fn push_default(&mut self) -> Result<()> {
        let Some(last_offset) = self.offsets.last() else {
            return Err(Error(String::from("invalid state")));
        };
        self.offsets.push(*last_offset);
        Ok(())
    }

    fn build_array(&mut self) -> Result<marrow::array::Array> {
        Ok(O::ARRAY_VARIANT(ListArray {
            validity: None,
            offsets: std::mem::replace(&mut self.offsets, vec![O::default()]),
            meta: FieldMeta {
                name: String::from("element"),
                ..Default::default()
            },
            elements: Box::new(self.builder.build_array()?),
        }))
    }
}

impl<T, O: Offset, B: ArrayPush<T>> ArrayPush<[T]> for GenericListBuilder<O, B> {
    fn push_value(&mut self, value: &[T]) -> Result<()> {
        let Some(last_offset) = self.offsets.last().copied() else {
            return Err(Error(String::from("invalid state")));
        };

        let mut pushed = O::default();
        for item in value {
            self.builder.push_value(item)?;
            pushed = pushed + O::ONE;
        }

        self.offsets.push(last_offset + pushed);
        Ok(())
    }
}

pub struct ListBuilder<B>(GenericListBuilder<i32, B>);

impl<B> ListBuilder<B> {
    pub fn new(builder: B) -> Self {
        Self(GenericListBuilder::new(builder))
    }
}

impl<B: ArrayBuilder> ArrayBuilder for ListBuilder<B> {
    fn push_default(&mut self) -> Result<()> {
        self.0.push_default()
    }

    fn build_array(&mut self) -> Result<Array> {
        self.0.build_array()
    }
}

impl<T, B: ArrayPush<T>> ArrayPush<[T]> for ListBuilder<B> {
    fn push_value(&mut self, value: &[T]) -> Result<()> {
        self.0.push_value(value)
    }
}

impl<T, B: ArrayPush<T>> ArrayPush<Vec<T>> for ListBuilder<B> {
    fn push_value(&mut self, value: &Vec<T>) -> Result<()> {
        self.0.push_value(value.as_slice())
    }
}

pub struct LargeListBuilder<B>(GenericListBuilder<i64, B>);

impl<B> LargeListBuilder<B> {
    pub fn new(builder: B) -> Self {
        Self(GenericListBuilder::new(builder))
    }
}

impl<B: ArrayBuilder> ArrayBuilder for LargeListBuilder<B> {
    fn push_default(&mut self) -> Result<()> {
        self.0.push_default()
    }

    fn build_array(&mut self) -> Result<Array> {
        self.0.build_array()
    }
}

impl<T, B: ArrayPush<T>> ArrayPush<[T]> for LargeListBuilder<B> {
    fn push_value(&mut self, value: &[T]) -> Result<()> {
        self.0.push_value(value)
    }
}

impl<T, B: ArrayPush<T>> ArrayPush<Vec<T>> for LargeListBuilder<B> {
    fn push_value(&mut self, value: &Vec<T>) -> Result<()> {
        self.0.push_value(value.as_slice())
    }
}

impl<T: DefaultArrayBuilder> DefaultArrayBuilder for Vec<T> {
    type ArrayBuilder = LargeListBuilder<T::ArrayBuilder>;

    fn default_builder() -> Self::ArrayBuilder {
        LargeListBuilder::new(T::default_builder())
    }
}

impl<T: DefaultArrayBuilder> DefaultArrayBuilder for [T] {
    type ArrayBuilder = LargeListBuilder<T::ArrayBuilder>;

    fn default_builder() -> Self::ArrayBuilder {
        LargeListBuilder::new(T::default_builder())
    }
}
