use marrow::{
    datatypes::{DataType, Field, TimeUnit, UnionMode},
    types::f16,
};
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
fn newtype() {
    #[derive(TypeInfo)]
    #[allow(dead_code)]
    struct S(f16);

    assert_eq!(
        Context::default().get_data_type::<S>(),
        Ok(DataType::Float16)
    );
}

#[test]
fn tuple() {
    #[derive(TypeInfo)]
    #[allow(dead_code)]
    struct S(u8, [u8; 4]);

    assert_eq!(
        Context::default().get_data_type::<S>(),
        Ok(DataType::Struct(vec![
            Field {
                name: String::from("0"),
                data_type: DataType::UInt8,
                ..Field::default()
            },
            Field {
                name: String::from("1"),
                data_type: DataType::FixedSizeBinary(4),
                ..Field::default()
            },
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

    fn timestamp_field<T>(_: &Context, name: &str) -> Field {
        Field {
            name: String::from(name),
            data_type: DataType::Timestamp(TimeUnit::Millisecond, None),
            nullable: false,
            metadata: Default::default(),
        }
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
    #[allow(dead_code)]
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

#[test]
fn new_tuple_enum() {
    #[derive(TypeInfo)]
    #[allow(dead_code)]
    enum Enum {
        Int64(i64),
        Tuple(i8, u32),
    }

    assert_eq!(
        Context::default().get_data_type::<Enum>(),
        Ok(DataType::Union(
            vec![
                (
                    0,
                    Field {
                        name: String::from("Int64"),
                        data_type: DataType::Int64,
                        ..Field::default()
                    }
                ),
                (
                    1,
                    Field {
                        name: String::from("Tuple"),
                        data_type: DataType::Struct(vec![
                            Field {
                                name: String::from("0"),
                                data_type: DataType::Int8,
                                ..Field::default()
                            },
                            Field {
                                name: String::from("1"),
                                data_type: DataType::UInt32,
                                ..Field::default()
                            },
                        ]),
                        ..Field::default()
                    }
                ),
            ],
            UnionMode::Dense
        ))
    );
}

#[test]
fn new_struct_enum() {
    #[derive(TypeInfo)]
    #[allow(dead_code)]
    enum Enum {
        Int64(i64),
        Struct { a: f32, b: String },
    }

    assert_eq!(
        Context::default().get_data_type::<Enum>(),
        Ok(DataType::Union(
            vec![
                (
                    0,
                    Field {
                        name: String::from("Int64"),
                        data_type: DataType::Int64,
                        ..Field::default()
                    }
                ),
                (
                    1,
                    Field {
                        name: String::from("Struct"),
                        data_type: DataType::Struct(vec![
                            Field {
                                name: String::from("a"),
                                data_type: DataType::Float32,
                                ..Field::default()
                            },
                            Field {
                                name: String::from("b"),
                                data_type: DataType::LargeUtf8,
                                ..Field::default()
                            },
                        ]),
                        ..Field::default()
                    }
                ),
            ],
            UnionMode::Dense
        ))
    );
}
