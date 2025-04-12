use marrow::datatypes::{DataType, Field};

use crate::TypeInfo;

impl TypeInfo for bigdecimal::BigDecimal {
    fn get_field(context: crate::Context<'_>) -> crate::Result<marrow::datatypes::Field> {
        Ok(Field {
            name: String::from(context.get_name()),
            // TODO: find better defaults
            data_type: DataType::Decimal128(5, 5),
            ..Default::default()
        })
    }
}
