use marrow::{
    datatypes::{DataType, Field},
    types::f16,
};

use crate::{
    Result,
    types::{Context, DefaultArrayType},
};

use super::utils::new_string_field;

macro_rules! define_primitive {
    ($(($ty:ty, $dt:expr),)*) => {
        $(
            impl DefaultArrayType for $ty {
                fn get_field(context: Context<'_>) -> Result<Field> {
                    Ok(Field {
                        name: context.get_name().to_owned(),
                        data_type: $dt,
                        ..Field::default()
                    })
                }
            }
        )*
    };
}

define_primitive!(
    (bool, DataType::Boolean),
    (u8, DataType::UInt8),
    (u16, DataType::UInt16),
    (u32, DataType::UInt32),
    (u64, DataType::UInt64),
    (i8, DataType::Int8),
    (i16, DataType::Int16),
    (i32, DataType::Int32),
    (i64, DataType::Int64),
    (f16, DataType::Float16),
    (f32, DataType::Float32),
    (f64, DataType::Float64),
    (char, DataType::UInt32),
);

impl DefaultArrayType for () {
    fn get_field(context: Context<'_>) -> Result<Field> {
        let _ = context;
        Ok(Field {
            name: context.get_name().to_owned(),
            data_type: DataType::Null,
            nullable: true,
            metadata: Default::default(),
        })
    }
}

impl DefaultArrayType for str {
    fn get_field(context: Context<'_>) -> Result<Field> {
        Ok(new_string_field(context))
    }
}

impl<T: DefaultArrayType> DefaultArrayType for &T {
    fn get_field(context: Context<'_>) -> Result<Field> {
        T::get_field(context)
    }
}

impl<T: DefaultArrayType> DefaultArrayType for &mut T {
    fn get_field(context: Context<'_>) -> Result<Field> {
        T::get_field(context)
    }
}
