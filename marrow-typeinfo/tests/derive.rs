use marrow::datatypes::{DataType, Field, UnionMode};
use marrow_typeinfo::TypeInfo;

#[test]
fn example() {
    #[derive(TypeInfo)]
    #[allow(dead_code)]
    struct S {
        a: i64,
        b: [u8; 4],
    }

    assert_eq!(
        <S as TypeInfo>::get_data_type(&Default::default()),
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
fn fieldless_union() {
    #[derive(TypeInfo)]
    #[allow(dead_code)]
    enum E {
        A,
        B,
        C,
    }

    assert_eq!(
        <E as TypeInfo>::get_data_type(&Default::default()),
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
