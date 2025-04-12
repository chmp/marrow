use marrow::datatypes::DataType;

use crate::{Options, get_data_type};

#[test]
fn examples() {
    assert_eq!(
        get_data_type::<i64>(&Options::default()),
        Ok(DataType::Int64)
    );
    assert_eq!(
        get_data_type::<[u8; 8]>(&Options::default()),
        Ok(DataType::FixedSizeBinary(8))
    );
}
