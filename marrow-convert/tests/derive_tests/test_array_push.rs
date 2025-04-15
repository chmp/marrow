use marrow::datatypes::FieldMeta;
use marrow_convert::builder::{ArrayBuilder, ArrayPush};

#[test]
fn example() {
    #[derive(marrow_convert::builder::ArrayPush)]
    struct S {
        a: i32,
        b: i64,
    }

    let mut builder = marrow_convert::builder::compound::StructBuilder {
        len: 0,
        meta: vec![
            FieldMeta {
                name: String::from("a"),
                ..Default::default()
            },
            FieldMeta {
                name: String::from("b"),
                ..Default::default()
            },
        ],
        children: (
            marrow_convert::builder::Int32Builder::default(),
            marrow_convert::builder::Int64Builder::default(),
        ),
    };

    builder.push_value(&S { a: 1, b: -1 }).unwrap();
    builder.push_value(&S { a: 2, b: -2 }).unwrap();
    builder.push_value(&S { a: 3, b: -3 }).unwrap();

    let array = builder.build_array().unwrap();
    // TODO: check resulting array
    std::mem::drop(array);
}
