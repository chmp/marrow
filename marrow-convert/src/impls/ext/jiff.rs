use marrow::datatypes::{DataType, Field, TimeUnit};

use crate::TypeInfo;

impl TypeInfo for jiff::civil::Date {
    fn get_field(context: crate::Context<'_>) -> crate::Result<Field> {
        Ok(Field {
            name: String::from(context.get_name()),
            data_type: DataType::Date32,
            ..Default::default()
        })
    }
}

impl TypeInfo for jiff::civil::Time {
    fn get_field(context: crate::Context<'_>) -> crate::Result<Field> {
        Ok(Field {
            name: String::from(context.get_name()),
            data_type: DataType::Time32(TimeUnit::Millisecond),
            ..Default::default()
        })
    }
}

impl TypeInfo for jiff::Span {
    fn get_field(context: crate::Context<'_>) -> crate::Result<Field> {
        Ok(Field {
            name: String::from(context.get_name()),
            data_type: DataType::Duration(TimeUnit::Millisecond),
            ..Default::default()
        })
    }
}

impl TypeInfo for jiff::Timestamp {
    fn get_field(context: crate::Context<'_>) -> crate::Result<Field> {
        Ok(Field {
            name: String::from(context.get_name()),
            data_type: DataType::Timestamp(TimeUnit::Millisecond, None),
            ..Default::default()
        })
    }
}
