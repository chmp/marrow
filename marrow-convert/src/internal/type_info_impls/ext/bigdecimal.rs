use marrow::datatypes::{DataType, Field};

use crate::{
    Result,
    types::{Context, DefaultArrayType},
};

impl DefaultArrayType for bigdecimal::BigDecimal {
    fn get_field(context: Context<'_>) -> Result<marrow::datatypes::Field> {
        Ok(Field {
            name: String::from(context.get_name()),
            // TODO: find better defaults
            data_type: DataType::Decimal128(5, 5),
            ..Default::default()
        })
    }
}
