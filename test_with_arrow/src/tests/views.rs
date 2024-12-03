use marrow::datatypes::DataType;

#[test]
fn view_types() {
    assert_eq!(
        DataType::try_from(&arrow_schema::DataType::Utf8View).unwrap(),
        DataType::Utf8View
    );
    assert_eq!(
        DataType::try_from(&arrow_schema::DataType::BinaryView).unwrap(),
        DataType::BinaryView
    );

    assert_eq!(
        arrow_schema::DataType::try_from(&DataType::Utf8View).unwrap(),
        arrow_schema::DataType::Utf8View
    );
    assert_eq!(
        arrow_schema::DataType::try_from(&DataType::BinaryView).unwrap(),
        arrow_schema::DataType::BinaryView
    );
}
