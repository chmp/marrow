use marrow::array::Array;

use crate::Result;

pub mod list;
pub mod option;
pub mod primitive;
pub mod r#struct;
pub mod union;

pub trait ArrayBuilder {
    fn push_default(&mut self) -> Result<()>;
    fn build_array(&mut self) -> Result<Array>;
}

pub trait ArrayPush<T: ?Sized>: ArrayBuilder {
    fn push_value(&mut self, value: &T) -> Result<()>;
}

impl<T, B: ArrayPush<T>> ArrayPush<&T> for B {
    fn push_value(&mut self, value: &&T) -> Result<()> {
        self.push_value(*value)
    }
}

impl<T, B: ArrayPush<T>> ArrayPush<&mut T> for B {
    fn push_value(&mut self, value: &&mut T) -> Result<()> {
        self.push_value(*value)
    }
}

pub trait DefaultArrayBuilder {
    type ArrayBuilder: ArrayBuilder;

    fn default_builder() -> Self::ArrayBuilder;
}

impl<T: DefaultArrayBuilder> DefaultArrayBuilder for &T {
    type ArrayBuilder = T::ArrayBuilder;

    fn default_builder() -> Self::ArrayBuilder {
        T::default_builder()
    }
}

impl<T: DefaultArrayBuilder> DefaultArrayBuilder for &mut T {
    type ArrayBuilder = T::ArrayBuilder;

    fn default_builder() -> Self::ArrayBuilder {
        T::default_builder()
    }
}
