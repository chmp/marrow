use marrow::datatypes::{DataType, Field};

use crate::{
    Result,
    internal::type_info::{DefaultStringType, LargeList},
    types::{Context, DefaultArrayType},
};

pub fn new_field(name: &str, data_type: DataType) -> Field {
    Field {
        name: name.to_owned(),
        data_type,
        nullable: false,
        metadata: Default::default(),
    }
}

pub fn new_string_field(context: Context<'_>) -> Field {
    let ty = if let Some(DefaultStringType(ty)) = context.get_options().get() {
        ty.clone()
    } else {
        DataType::LargeUtf8
    };
    new_field(context.get_name(), ty)
}

pub fn new_list_field<T: DefaultArrayType>(context: Context<'_>) -> Result<Field> {
    let larget_list = if let Some(LargeList(large_list)) = context.get_options().get() {
        *large_list
    } else {
        false
    };

    let base_field = context.get_field::<T>("element")?;

    Ok(Field {
        name: context.get_name().to_owned(),
        data_type: if larget_list {
            DataType::LargeList(Box::new(base_field))
        } else {
            DataType::List(Box::new(base_field))
        },
        nullable: false,
        metadata: Default::default(),
    })
}

pub fn new_map_field<K: DefaultArrayType, V: DefaultArrayType>(
    context: Context<'_>,
) -> Result<Field> {
    let key_field = context.get_field::<K>("key")?;
    let value_field = context.get_field::<V>("value")?;
    let entry_field = new_field("entry", DataType::Struct(vec![key_field, value_field]));

    Ok(new_field(
        context.get_name(),
        DataType::Map(Box::new(entry_field), false),
    ))
}
