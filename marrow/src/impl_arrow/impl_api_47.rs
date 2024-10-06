// Implement the api starting from `arrow=47`
use crate::view::FixedSizeBinaryView;

fn wrap_fixed_size_binary_array(array: &arrow_array::FixedSizeBinaryArray) -> Result<View<'_>> {
    Ok(View::FixedSizeBinary(FixedSizeBinaryView {
        n: array.value_length(),
        validity: get_bits_with_offset(array),
        data: array.value_data(),
    }))
}

include!("impl_api_base.rs");
