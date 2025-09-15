use chrono::Utc;
use marrow::datatypes::{DataType, Field, TimeUnit};

use crate::{
    Result,
    types::{Context, DefaultArrayType},
};

impl DefaultArrayType for chrono::NaiveDate {
    fn get_field(context: Context<'_>) -> Result<marrow::datatypes::Field> {
        Ok(Field {
            name: String::from(context.get_name()),
            data_type: DataType::Date32,
            ..Default::default()
        })
    }
}

impl DefaultArrayType for chrono::NaiveTime {
    fn get_field(context: Context<'_>) -> Result<marrow::datatypes::Field> {
        Ok(Field {
            name: String::from(context.get_name()),
            data_type: DataType::Time32(TimeUnit::Millisecond),
            ..Default::default()
        })
    }
}

impl DefaultArrayType for chrono::NaiveDateTime {
    fn get_field(context: Context<'_>) -> Result<marrow::datatypes::Field> {
        Ok(Field {
            name: String::from(context.get_name()),
            data_type: DataType::Timestamp(TimeUnit::Millisecond, None),
            ..Default::default()
        })
    }
}

impl DefaultArrayType for chrono::DateTime<Utc> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        Ok(Field {
            name: String::from(context.get_name()),
            data_type: DataType::Timestamp(TimeUnit::Millisecond, Some(String::from("UTC"))),
            ..Default::default()
        })
    }
}
