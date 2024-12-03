// Implement the api starting from `arrow=37`
fn wrap_fixed_size_binary_array(_array: &arrow_array::FixedSizeBinaryArray) -> Result<View<'_>> {
    fail!(
        ErrorKind::Unsupported,
        "FixedSizeBinary arrays are not supported for arrow<=46"
    );
}

fn convert_extra_datatype(
    data_type: &arrow_schema::DataType,
) -> Result<crate::datatypes::DataType> {
    fail!(
        ErrorKind::Unsupported,
        "Unsupported arrow data type {data_type}"
    );
}

fn build_utf8_view_datatype() -> Result<arrow_schema::DataType> {
    fail!(ErrorKind::Unsupported, "Unsupported data type Utf8View");
}

fn build_binary_view_datatype() -> Result<arrow_schema::DataType> {
    fail!(ErrorKind::Unsupported, "Unsupported data type BinaryView");
}

include!("impl_api_base.rs");
