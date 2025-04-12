use chrono::Utc;
use marrow::datatypes::{DataType, Field, TimeUnit};

use crate::TypeInfo;

impl TypeInfo for chrono::NaiveDate {
    fn get_field(context: crate::Context<'_>) -> crate::Result<marrow::datatypes::Field> {
        Ok(Field {
            name: String::from(context.get_name()),
            data_type: DataType::Date32,
            ..Default::default()
        })
    }
}

impl TypeInfo for chrono::NaiveTime {
    fn get_field(context: crate::Context<'_>) -> crate::Result<marrow::datatypes::Field> {
        Ok(Field {
            name: String::from(context.get_name()),
            data_type: DataType::Time32(TimeUnit::Millisecond),
            ..Default::default()
        })
    }
}

impl TypeInfo for chrono::NaiveDateTime {
    fn get_field(context: crate::Context<'_>) -> crate::Result<marrow::datatypes::Field> {
        Ok(Field {
            name: String::from(context.get_name()),
            data_type: DataType::Timestamp(TimeUnit::Millisecond, None),
            ..Default::default()
        })
    }
}

impl TypeInfo for chrono::DateTime<Utc> {
    fn get_field(context: crate::Context<'_>) -> crate::Result<Field> {
        Ok(Field {
            name: String::from(context.get_name()),
            data_type: DataType::Timestamp(TimeUnit::Millisecond, Some(String::from("UTC"))),
            ..Default::default()
        })
    }
}
