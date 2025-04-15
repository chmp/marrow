use marrow::{
    array::{Array, StructArray},
    datatypes::FieldMeta,
};

use crate::{Error, Result};

use super::ArrayBuilder;

// TODO: add simple doc test showing how to implement a custom impl
/// Support to build struct builders
///
/// When pushing a value the following invariants need to be observed:
///
/// - A value must be pushed to each child field
/// - The `len` field must be incremented
///
pub struct StructBuilder<C> {
    pub meta: Vec<FieldMeta>,
    pub len: usize,
    pub children: C,
}

macro_rules! impl_struct_builder {
    ($($el:ident,)*) => {
        #[allow(non_snake_case, clippy::vec_init_then_push)]
        impl<$($el: ArrayBuilder),*> ArrayBuilder for StructBuilder<($($el,)*)> {
            fn push_default(&mut self) -> Result<()> {
                let ($($el,)*)  = &mut self.children;
                self.len += 1;
                $($el.push_default()?;)*
                Ok(())
            }

            fn build_array(&mut self) -> Result<Array> {
                let ($($el,)*) = &mut self.children;
                let mut arrays = Vec::new();
                // TODO: ensure all builders are called?
                $(arrays.push($el.build_array()?);)*

                if arrays.len() != self.meta.len() {
                    return Err(Error(String::from("Not matching number of meta and children")));
                }

                let fields = std::iter::zip(&self.meta, arrays).map(|(meta, array)| (meta.clone(), array)).collect();

                Ok(Array::Struct(StructArray {
                    len: self.len,
                    validity: None,
                    fields,
                }))
            }
        }
    };
}

// TODO: is a struct without fields valid?
impl_struct_builder!(A,);
impl_struct_builder!(A, B,);
impl_struct_builder!(A, B, C,);
impl_struct_builder!(A, B, C, D,);
impl_struct_builder!(A, B, C, D, E,);
impl_struct_builder!(A, B, C, D, E, F,);
impl_struct_builder!(A, B, C, D, E, F, G,);
impl_struct_builder!(A, B, C, D, E, F, G, H,);
impl_struct_builder!(A, B, C, D, E, F, G, H, I,);
impl_struct_builder!(A, B, C, D, E, F, G, H, I, J,);
impl_struct_builder!(A, B, C, D, E, F, G, H, I, J, K,);
impl_struct_builder!(A, B, C, D, E, F, G, H, I, J, K, L,);
impl_struct_builder!(A, B, C, D, E, F, G, H, I, J, K, L, M,);
impl_struct_builder!(A, B, C, D, E, F, G, H, I, J, K, L, M, N,);
impl_struct_builder!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O,);
impl_struct_builder!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,);

#[test]
fn struct_example() {
    use super::{ArrayPush, DefaultArrayBuilder};

    struct S {
        a: i8,
        b: i32,
    }

    // move into derive(ArrayPush)
    // Allows to customize the builder
    const _: () = {
        impl<A: ArrayPush<i8>, B: ArrayPush<i32>> ArrayPush<S> for StructBuilder<(A, B)> {
            fn push_value(&mut self, value: &S) -> Result<()> {
                self.len += 1;
                self.children.0.push_value(&value.a)?;
                self.children.1.push_value(&value.b)?;
                Ok(())
            }
        }
    };

    // move into derive(DefaultBuilder)
    const _: () = {
        struct Builder(
            StructBuilder<(
                <i8 as DefaultArrayBuilder>::ArrayBuilder,
                <i32 as DefaultArrayBuilder>::ArrayBuilder,
            )>,
        );

        impl ArrayBuilder for Builder {
            fn push_default(&mut self) -> Result<()> {
                self.0.push_default()
            }

            fn build_array(&mut self) -> Result<Array> {
                self.0.build_array()
            }
        }

        impl DefaultArrayBuilder for S {
            type ArrayBuilder = Builder;

            fn default_builder() -> Self::ArrayBuilder {
                Builder(StructBuilder {
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
                        (<i8 as DefaultArrayBuilder>::default_builder()),
                        (<i32 as DefaultArrayBuilder>::default_builder()),
                    ),
                })
            }
        }

        // NOTE: implement separately to allow independent derives
        impl ArrayPush<S> for Builder {
            fn push_value(&mut self, value: &S) -> Result<()> {
                self.0.len += 1;
                self.0.children.0.push_value(&value.a)?;
                self.0.children.1.push_value(&value.b)?;
                Ok(())
            }
        }
    };

    // the public API
    let mut builder = S::default_builder();
    builder.push_value(&S { a: 0, b: -21 }).unwrap();
    builder.push_value(&S { a: 1, b: -42 }).unwrap();
    let array = builder.build_array().unwrap();

    let [(_, a), (_, b)] = array.into_struct_fields().expect("invalid array type");
    let a = a.into_int8().expect("invalid array type");
    let b = b.into_int32().expect("invalid array type");

    assert_eq!(a.values, vec![0, 1]);
    assert_eq!(b.values, vec![-21, -42]);
}
