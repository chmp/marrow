fn wrap_fixed_size_binary_array(
    _array: &arrow_array::FixedSizeBinaryArray,
) -> Result<View<'_>> {
    fail!(
        ErrorKind::Unsupported,
        "FixedSizeBinary arrays are not supported for arrow<=46"
    );
}

include!("impl_api_base.rs");
