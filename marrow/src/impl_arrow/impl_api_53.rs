// Implement the api starting from `arrow=47`
use crate::view::FixedSizeBinaryView;

fn wrap_fixed_size_binary_array(array: &arrow_array::FixedSizeBinaryArray) -> Result<View<'_>> {
    Ok(View::FixedSizeBinary(FixedSizeBinaryView {
        n: array.value_length(),
        validity: get_bits_with_offset(array),
        data: array.value_data(),
    }))
}

fn convert_extra_datatype(
    data_type: &arrow_schema::DataType,
) -> Result<crate::datatypes::DataType> {
    match data_type {
        arrow_schema::DataType::Utf8View => Ok(crate::datatypes::DataType::Utf8View),
        arrow_schema::DataType::BinaryView => Ok(crate::datatypes::DataType::BinaryView),
        data_type => fail!(
            ErrorKind::Unsupported,
            "Unsupported arrow data type {data_type}"
        ),
    }
}

fn build_utf8_view_datatype() -> Result<arrow_schema::DataType> {
    Ok(arrow_schema::DataType::Utf8View)
}

fn build_binary_view_datatype() -> Result<arrow_schema::DataType> {
    Ok(arrow_schema::DataType::BinaryView)
}

include!("impl_api_base.rs");
