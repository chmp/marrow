use marrow_convert::builder::{ArrayBuilder, ArrayPush, DefaultArrayBuilder};

#[test]
fn example() {
    #[derive(DefaultArrayBuilder)]
    struct S {
        a: i32,
        b: i64,
    }

    let mut builder = S::default_builder();

    builder.push_value(&S { a: 1, b: -1 }).unwrap();
    builder.push_value(&S { a: 2, b: -2 }).unwrap();
    builder.push_value(&S { a: 3, b: -3 }).unwrap();

    let array = builder.build_array().unwrap();
    // TODO: check resulting array
    std::mem::drop(array);
}
