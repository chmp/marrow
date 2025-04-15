use marrow::array::Array;

use crate::{Error, Result};

use super::{ArrayBuilder, ArrayPush, DefaultArrayBuilder};

pub struct OptionBuilder<B> {
    len: usize,
    validity: Vec<u8>,
    builder: B,
}

impl<B> OptionBuilder<B> {
    pub fn new(builder: B) -> Self {
        Self {
            len: 0,
            validity: Vec::new(),
            builder,
        }
    }
}

impl<B: ArrayBuilder> ArrayBuilder for OptionBuilder<B> {
    fn push_default(&mut self) -> Result<()> {
        marrow::bits::push(&mut self.validity, &mut self.len, false);
        self.builder.push_default()?;
        Ok(())
    }

    fn build_array(&mut self) -> Result<Array> {
        let array = self.builder.build_array()?;
        let validity = std::mem::take(&mut self.validity);
        let _ = std::mem::take(&mut self.len);
        with_validity(array, validity)
    }
}

impl<T, B: ArrayPush<T>> ArrayPush<Option<T>> for OptionBuilder<B> {
    fn push_value(&mut self, value: &Option<T>) -> Result<()> {
        match value {
            Some(value) => {
                marrow::bits::push(&mut self.validity, &mut self.len, true);
                self.builder.push_value(value)
            }
            None => self.push_default(),
        }
    }
}

impl<T: DefaultArrayBuilder> DefaultArrayBuilder for Option<T> {
    type ArrayBuilder = OptionBuilder<T::ArrayBuilder>;

    fn default_builder() -> Self::ArrayBuilder {
        OptionBuilder::new(T::default_builder())
    }
}

fn with_validity(array: Array, validity: Vec<u8>) -> Result<Array> {
    // TODO: check compatibility
    match array {
        Array::Null(array) => Ok(Array::Null(array)),
        Array::Boolean(mut array) => {
            array.validity = Some(validity);
            Ok(Array::Boolean(array))
        }
        // TODO: add more ..
        _ => Err(Error(String::from("Cannot set valditiy for array"))),
    }
}
