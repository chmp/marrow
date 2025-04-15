use std::collections::HashMap;

use marrow::datatypes::{DataType, Field};

use crate::{
    Result,
    types::{Context, DefaultArrayType},
};

impl DefaultArrayType for uuid::Uuid {
    fn get_field(context: Context<'_>) -> Result<Field> {
        let mut metadata = HashMap::new();
        metadata.insert("ARROW:extension:name".into(), "arrow.uuid".into());
        metadata.insert("ARROW:extension:metadata".into(), String::new());

        Ok(Field {
            name: String::from(context.get_name()),
            data_type: DataType::FixedSizeBinary(16),
            metadata,
            ..Default::default()
        })
    }
}
