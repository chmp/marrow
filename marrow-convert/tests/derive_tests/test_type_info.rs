use marrow::{
    datatypes::{DataType, Field, TimeUnit, UnionMode},
    types::f16,
};
use marrow_convert::{
    Result,
    types::{Context, DefaultArrayType, Options},
};

#[test]
fn example() {
    #[derive(DefaultArrayType)]
    #[allow(dead_code)]
    struct S {
        a: i64,
        b: [u8; 4],
    }

    assert_eq!(
        marrow_convert::types::get_data_type::<S>(&Options::default()),
        Ok(DataType::Struct(vec![
            Field {
                name: String::from("a"),
                data_type: DataType::Int64,
                ..Default::default()
            },
            Field {
                name: String::from("b"),
                data_type: DataType::FixedSizeBinary(4),
                ..Default::default()
            }
        ]))
    );
}

#[test]
fn overwrites() {
    #[derive(DefaultArrayType)]
    #[allow(dead_code)]
    struct S {
        a: i64,
        b: [u8; 4],
    }

    assert_eq!(
        marrow_convert::types::get_data_type::<S>(&Options::default().overwrite(
            "$.b",
            Field {
                data_type: DataType::Binary,
                ..Field::default()
            }
        )),
        Ok(DataType::Struct(vec![
            Field {
                name: String::from("a"),
                data_type: DataType::Int64,
                ..Default::default()
            },
            Field {
                name: String::from("b"),
                data_type: DataType::Binary,
                ..Default::default()
            }
        ]))
    );
}

#[test]
fn newtype() {
    #[derive(DefaultArrayType)]
    #[allow(dead_code)]
    struct S(f16);

    assert_eq!(
        marrow_convert::types::get_data_type::<S>(&Options::default()),
        Ok(DataType::Float16)
    );
}

#[test]
fn tuple() {
    #[derive(DefaultArrayType)]
    #[allow(dead_code)]
    struct S(u8, [u8; 4]);

    assert_eq!(
        marrow_convert::types::get_data_type::<S>(&Options::default()),
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
    #[derive(DefaultArrayType)]
    #[allow(dead_code)]
    struct S {
        #[marrow(with = "timestamp_field")]
        a: i64,
        b: [u8; 4],
    }

    fn timestamp_field<T>(context: Context<'_>) -> Result<Field> {
        Ok(Field {
            name: String::from(context.get_name()),
            data_type: DataType::Timestamp(TimeUnit::Millisecond, None),
            ..Default::default()
        })
    }

    assert_eq!(
        marrow_convert::types::get_data_type::<S>(&Options::default()),
        Ok(DataType::Struct(vec![
            Field {
                name: String::from("a"),
                data_type: DataType::Timestamp(TimeUnit::Millisecond, None),
                ..Default::default()
            },
            Field {
                name: String::from("b"),
                data_type: DataType::FixedSizeBinary(4),
                ..Default::default()
            }
        ]))
    );
}

#[test]
fn fieldless_union() {
    #[derive(DefaultArrayType)]
    #[allow(dead_code)]
    enum E {
        A,
        B,
        C,
    }

    assert_eq!(
        marrow_convert::types::get_data_type::<E>(&Options::default()),
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
    #[derive(DefaultArrayType)]
    #[allow(dead_code)]
    enum Enum {
        Struct(Struct),
        Int64(i64),
    }

    #[derive(DefaultArrayType)]
    #[allow(dead_code)]
    struct Struct {
        a: bool,
        b: (),
    }

    assert_eq!(
        marrow_convert::types::get_data_type::<Enum>(&Options::default()),
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
                                ..Default::default()
                            },
                            Field {
                                name: String::from("b"),
                                data_type: DataType::Null,
                                nullable: true,
                                ..Default::default()
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
                        ..Default::default()
                    }
                ),
            ],
            UnionMode::Dense
        ))
    );
}

#[test]
fn new_tuple_enum() {
    #[derive(DefaultArrayType)]
    #[allow(dead_code)]
    enum Enum {
        Int64(i64),
        Tuple(i8, u32),
    }

    assert_eq!(
        marrow_convert::types::get_data_type::<Enum>(&Options::default()),
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
    #[derive(DefaultArrayType)]
    #[allow(dead_code)]
    enum Enum {
        Int64(i64),
        Struct { a: f32, b: String },
    }

    assert_eq!(
        marrow_convert::types::get_data_type::<Enum>(&Options::default()),
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

#[test]
fn const_generics() {
    #[derive(DefaultArrayType)]
    #[allow(unused)]
    struct Struct<const N: usize> {
        data: [u8; N],
    }

    assert_eq!(
        marrow_convert::types::get_data_type::<Struct<4>>(&Options::default()),
        Ok(DataType::Struct(vec![Field {
            name: String::from("data"),
            data_type: DataType::FixedSizeBinary(4),
            nullable: false,
            metadata: Default::default(),
        },]))
    );
}

#[test]
fn liftime_generics() {
    #[derive(DefaultArrayType)]
    #[allow(unused)]
    struct Struct<'a, 'b> {
        a: &'a u8,
        b: &'b u16,
    }

    assert_eq!(
        marrow_convert::types::get_data_type::<Struct>(&Options::default()),
        Ok(DataType::Struct(vec![
            Field {
                name: String::from("a"),
                data_type: DataType::UInt8,
                ..Default::default()
            },
            Field {
                name: String::from("b"),
                data_type: DataType::UInt16,
                ..Default::default()
            },
        ]))
    );
}

#[test]
fn liftime_generics_with_bounds() {
    #[derive(DefaultArrayType)]
    #[allow(unused)]
    struct Struct<'a, 'b: 'a> {
        a: &'a u8,
        b: &'b u16,
    }

    assert_eq!(
        marrow_convert::types::get_data_type::<Struct>(&Options::default()),
        Ok(DataType::Struct(vec![
            Field {
                name: String::from("a"),
                data_type: DataType::UInt8,
                ..Default::default()
            },
            Field {
                name: String::from("b"),
                data_type: DataType::UInt16,
                ..Default::default()
            },
        ]))
    );
}

#[test]
fn liftime_generics_with_where_clause() {
    #[derive(DefaultArrayType)]
    #[allow(unused)]
    struct Struct<'a, 'b>
    where
        'a: 'b,
    {
        a: &'a u8,
        b: &'b u16,
    }

    assert_eq!(
        marrow_convert::types::get_data_type::<Struct>(&Options::default()),
        Ok(DataType::Struct(vec![
            Field {
                name: String::from("a"),
                data_type: DataType::UInt8,
                ..Default::default()
            },
            Field {
                name: String::from("b"),
                data_type: DataType::UInt16,
                ..Default::default()
            },
        ]))
    );
}

#[test]
fn enums_const_generics() {
    #[derive(DefaultArrayType)]
    #[allow(unused)]
    enum Enum<const N: usize> {
        Data([u8; N]),
    }

    assert_eq!(
        marrow_convert::types::get_data_type::<Enum<4>>(&Options::default()),
        Ok(DataType::Union(
            vec![(
                0,
                Field {
                    name: String::from("Data"),
                    data_type: DataType::FixedSizeBinary(4),
                    nullable: false,
                    metadata: Default::default(),
                }
            ),],
            UnionMode::Dense
        )),
    );
}

#[test]
fn enums_with_liftime_generics() {
    #[derive(DefaultArrayType)]
    #[allow(unused)]
    enum Enum<'a, 'b> {
        A(&'a u8),
        B(&'b u16),
    }

    assert_eq!(
        marrow_convert::types::get_data_type::<Enum>(&Options::default()),
        Ok(DataType::Union(
            vec![
                (
                    0,
                    Field {
                        name: String::from("A"),
                        data_type: DataType::UInt8,
                        ..Default::default()
                    }
                ),
                (
                    1,
                    Field {
                        name: String::from("B"),
                        data_type: DataType::UInt16,
                        ..Default::default()
                    }
                ),
            ],
            UnionMode::Dense
        ))
    );
}

#[test]
fn enum_liftime_generics_with_bounds() {
    #[derive(DefaultArrayType)]
    #[allow(unused)]
    enum Enum<'a, 'b: 'a> {
        A(&'a u8),
        B(&'b u16),
    }

    assert_eq!(
        marrow_convert::types::get_data_type::<Enum>(&Options::default()),
        Ok(DataType::Union(
            vec![
                (
                    0,
                    Field {
                        name: String::from("A"),
                        data_type: DataType::UInt8,
                        ..Default::default()
                    }
                ),
                (
                    1,
                    Field {
                        name: String::from("B"),
                        data_type: DataType::UInt16,
                        ..Default::default()
                    }
                ),
            ],
            UnionMode::Dense
        ))
    );
}

#[test]
fn enum_liftime_generics_with_where_clause() {
    #[derive(DefaultArrayType)]
    #[allow(unused)]
    enum Enum<'a, 'b>
    where
        'a: 'b,
    {
        A(&'a u8),
        B(&'b u16),
    }

    assert_eq!(
        marrow_convert::types::get_data_type::<Enum>(&Options::default()),
        Ok(DataType::Union(
            vec![
                (
                    0,
                    Field {
                        name: String::from("A"),
                        data_type: DataType::UInt8,
                        ..Default::default()
                    }
                ),
                (
                    1,
                    Field {
                        name: String::from("B"),
                        data_type: DataType::UInt16,
                        ..Default::default()
                    }
                ),
            ],
            UnionMode::Dense
        ))
    );
}
