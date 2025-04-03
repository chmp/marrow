use marrow::datatypes::{DataType, Field, TimeUnit, UnionMode};
use marrow_typeinfo::{Context, TypeInfo};

#[test]
fn example() {
    #[derive(TypeInfo)]
    #[allow(dead_code)]
    struct S {
        a: i64,
        b: [u8; 4],
    }

    assert_eq!(
        Context::default().get_data_type::<S>(),
        Ok(DataType::Struct(vec![
            Field {
                name: String::from("a"),
                data_type: DataType::Int64,
                nullable: false,
                metadata: Default::default(),
            },
            Field {
                name: String::from("b"),
                data_type: DataType::FixedSizeBinary(4),
                nullable: false,
                metadata: Default::default(),
            }
        ]))
    );
}

#[test]
fn customize() {
    #[derive(TypeInfo)]
    #[allow(dead_code)]
    struct S {
        #[marrow_type_info(with = "timestamp_field")]
        a: i64,
        b: [u8; 4],
    }

    assert_eq!(
        Context::default().get_data_type::<S>(),
        Ok(DataType::Struct(vec![
            Field {
                name: String::from("a"),
                data_type: DataType::Timestamp(TimeUnit::Millisecond, None),
                nullable: false,
                metadata: Default::default(),
            },
            Field {
                name: String::from("b"),
                data_type: DataType::FixedSizeBinary(4),
                nullable: false,
                metadata: Default::default(),
            }
        ]))
    );
}

// TODO: pass context
fn timestamp_field(_: &Context, name: &str) -> Field {
    Field {
        name: String::from(name),
        data_type: DataType::Timestamp(TimeUnit::Millisecond, None),
        nullable: false,
        metadata: Default::default(),
    }
}

#[test]
fn fieldless_union() {
    #[derive(TypeInfo)]
    #[allow(dead_code)]
    enum E {
        A,
        B,
        C,
    }

    assert_eq!(
        Context::default().get_data_type::<E>(),
        Ok(DataType::Union(
            vec![
                (
                    0,
                    Field {
                        name: String::from("A"),
                        data_type: DataType::Null,
                        nullable: true,
                        metadata: Default::default(),
                    }
                ),
                (
                    1,
                    Field {
                        name: String::from("B"),
                        data_type: DataType::Null,
                        nullable: true,
                        metadata: Default::default(),
                    }
                ),
                (
                    2,
                    Field {
                        name: String::from("C"),
                        data_type: DataType::Null,
                        nullable: true,
                        metadata: Default::default(),
                    }
                ),
            ],
            UnionMode::Dense
        ))
    );
}

#[test]
fn new_type_enum() {
    #[derive(TypeInfo)]
    #[allow(dead_code)]
    enum Enum {
        Struct(Struct),
        Int64(i64),
    }

    #[derive(TypeInfo)]
    struct Struct {
        a: bool,
        b: (),
    }

    assert_eq!(
        Context::default().get_data_type::<Enum>(),
        Ok(DataType::Union(
            vec![
                (
                    0,
                    Field {
                        name: String::from("Struct"),
                        data_type: DataType::Struct(vec![
                            Field {
                                name: String::from("a"),
                                data_type: DataType::Boolean,
                                nullable: false,
                                metadata: Default::default(),
                            },
                            Field {
                                name: String::from("b"),
                                data_type: DataType::Null,
                                nullable: true,
                                metadata: Default::default(),
                            },
                        ]),
                        nullable: false,
                        metadata: Default::default(),
                    }
                ),
                (
                    1,
                    Field {
                        name: String::from("Int64"),
                        data_type: DataType::Int64,
                        nullable: false,
                        metadata: Default::default(),
                    }
                ),
            ],
            UnionMode::Dense
        ))
    );
}
