use marrow::{
    array::{Array, UnionArray},
    datatypes::FieldMeta,
};

use crate::Result;

use crate::internal::util::TupleLen;

use super::ArrayBuilder;

/// Helper struct to simplify implementing sparse Union builders
///
/// When pushing a value the following invariants need to be observed:
///
/// - A  discriminator must be pushed to the `types` value
/// - A value must be pushed to each child field
#[derive(Debug)]
pub struct SparseUnionBuilder<C> {
    pub types: Vec<i8>,
    pub meta: Vec<FieldMeta>,
    pub children: C,
}

macro_rules! impl_sparse_union_builder {
    ($($el:ident,)*) => {
        #[allow(non_snake_case, clippy::vec_init_then_push)]
        impl<$($el: ArrayBuilder),*> ArrayBuilder for SparseUnionBuilder<($($el,)*)> {
            fn push_default(&mut self) -> Result<()> {
                let ($($el,)*) = &mut self.children;
                $($el.push_default()?;)*
                self.types.push(0);
                Ok(())
            }

            fn build_array(&mut self) -> Result<Array> {
                const {
                    assert!(<($($el,)*) as TupleLen>::LEN < (i8::MAX as usize));
                }

                let types = std::mem::take(&mut self.types);
                let mut arrays = Vec::new();
                let ($($el,)*) = &mut self.children;
                $(arrays.push($el.build_array()?);)*

                let fields = std::iter::zip(&self.meta, arrays)
                    .enumerate()
                    .map(|(i, (meta, array))| (i as i8, meta.clone(), array))
                    .collect();

                Ok(Array::Union(UnionArray {
                    types,
                    fields,
                    offsets: None,
                }))
            }
        }
    };
}

impl_sparse_union_builder!(A,);
impl_sparse_union_builder!(A, B,);
impl_sparse_union_builder!(A, B, C,);
impl_sparse_union_builder!(A, B, C, D,);
impl_sparse_union_builder!(A, B, C, D, E,);
impl_sparse_union_builder!(A, B, C, D, E, F,);
impl_sparse_union_builder!(A, B, C, D, E, F, G,);
impl_sparse_union_builder!(A, B, C, D, E, F, G, H,);
impl_sparse_union_builder!(A, B, C, D, E, F, G, H, I,);
impl_sparse_union_builder!(A, B, C, D, E, F, G, H, I, J,);
impl_sparse_union_builder!(A, B, C, D, E, F, G, H, I, J, K,);
impl_sparse_union_builder!(A, B, C, D, E, F, G, H, I, J, K, L,);
impl_sparse_union_builder!(A, B, C, D, E, F, G, H, I, J, K, L, M,);
impl_sparse_union_builder!(A, B, C, D, E, F, G, H, I, J, K, L, M, N,);
impl_sparse_union_builder!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O,);
impl_sparse_union_builder!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,);

/// Helper struct to simplify implementing dense Union builders
///
/// When pushing a value the following invariants need to be observed:
///
/// - A discriminator must be pushed to the `types` value
/// - A value must be pushed for the relevant variant
#[derive(Debug)]
pub struct DenseUnionBuilder<C> {
    pub types: DenseTypes,
    pub offsets: Vec<i8>,
    pub meta: Vec<FieldMeta>,
    pub children: C,
}

#[derive(Debug)]
pub struct DenseTypes {
    types: Vec<i8>,
    offsets: Vec<i32>,
    current_offset: Vec<i32>,
}

impl DenseTypes {
    pub fn new(num_types: usize) -> Self {
        Self {
            types: Vec::new(),
            offsets: Vec::new(),
            current_offset: vec![0; num_types],
        }
    }

    pub fn take(&mut self) -> Self {
        let num_types = self.current_offset.len();
        Self {
            types: std::mem::take(&mut self.types),
            offsets: std::mem::take(&mut self.offsets),
            current_offset: std::mem::replace(&mut self.current_offset, vec![0; num_types]),
        }
    }

    pub fn push(&mut self, variant: i8) -> Result<()> {
        assert!(variant >= 0);

        self.types.push(variant);
        self.offsets.push(self.current_offset[variant as usize]);
        self.current_offset[variant as usize] += 1;
        Ok(())
    }
}

macro_rules! impl_dense_union_builder {
    ($first:ident, $($el:ident,)*) => {
        #[allow(non_snake_case, clippy::vec_init_then_push)]
        impl<$first: ArrayBuilder $(, $el: ArrayBuilder)*> ArrayBuilder for DenseUnionBuilder<($first, $($el,)*)> {
            fn push_default(&mut self) -> Result<()> {
                #[allow(unused_variables)]
                let ($first, $($el,)*) = &mut self.children;
                $first.push_default()?;
                self.types.push(0)?;
                Ok(())
            }

            fn build_array(&mut self) -> Result<Array> {
                const {
                    assert!(<($first, $($el,)*) as TupleLen>::LEN < (i8::MAX as usize));
                }

                let DenseTypes { types, offsets, ..} = self.types.take();
                let mut arrays = Vec::new();
                let ($first, $($el,)*) = &mut self.children;
                arrays.push($first.build_array()?);
                $(arrays.push($el.build_array()?);)*

                let fields = std::iter::zip(&self.meta, arrays)
                    .enumerate()
                    .map(|(i, (meta, array))| (i as i8, meta.clone(), array))
                    .collect();

                Ok(Array::Union(UnionArray {
                    types,
                    offsets: Some(offsets),
                    fields,
                }))
            }
        }
    };
}

impl_dense_union_builder!(A,);
impl_dense_union_builder!(A, B,);
impl_dense_union_builder!(A, B, C,);
impl_dense_union_builder!(A, B, C, D,);
impl_dense_union_builder!(A, B, C, D, E,);
impl_dense_union_builder!(A, B, C, D, E, F,);
impl_dense_union_builder!(A, B, C, D, E, F, G,);
impl_dense_union_builder!(A, B, C, D, E, F, G, H,);
impl_dense_union_builder!(A, B, C, D, E, F, G, H, I,);
impl_dense_union_builder!(A, B, C, D, E, F, G, H, I, J,);
impl_dense_union_builder!(A, B, C, D, E, F, G, H, I, J, K,);
impl_dense_union_builder!(A, B, C, D, E, F, G, H, I, J, K, L,);
impl_dense_union_builder!(A, B, C, D, E, F, G, H, I, J, K, L, M,);
impl_dense_union_builder!(A, B, C, D, E, F, G, H, I, J, K, L, M, N,);
impl_dense_union_builder!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O,);
impl_dense_union_builder!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,);

#[test]
fn enum_example() {
    use super::{ArrayPush, DefaultArrayBuilder};

    enum Enum {
        A(i32),
        B(i64),
    }

    // TODO: push into derive(ArrayPush)
    const _: () = {
        impl<A: ArrayPush<i32>, B: ArrayPush<i64>> ArrayPush<Enum> for SparseUnionBuilder<(A, B)> {
            #[allow(non_snake_case)]
            fn push_value(&mut self, value: &Enum) -> Result<()> {
                match value {
                    Enum::A(inner) => {
                        self.types.push(0);
                        let (A, B) = &mut self.children;
                        A.push_value(inner)?;
                        B.push_default()?;
                    }
                    Enum::B(inner) => {
                        self.types.push(1);
                        let (A, B) = &mut self.children;
                        A.push_default()?;
                        B.push_value(inner)?;
                    }
                }
                Ok(())
            }
        }
    };

    // TODO: push into derive(DefaultArrayBuilder)
    const _: () = {
        struct Builder(
            SparseUnionBuilder<(
                <i32 as DefaultArrayBuilder>::ArrayBuilder,
                <i64 as DefaultArrayBuilder>::ArrayBuilder,
            )>,
        );

        #[allow(non_snake_case)]
        impl ArrayBuilder for Builder {
            fn push_default(&mut self) -> Result<()> {
                self.0.types.push(0);

                let (A, B) = &mut self.0.children;
                A.push_default()?;
                B.push_default()?;
                Ok(())
            }

            fn build_array(&mut self) -> Result<Array> {
                self.0.build_array()
            }
        }

        // TODO: in practice implement separately to allow indepdent derives
        impl ArrayPush<Enum> for Builder {
            fn push_value(&mut self, value: &Enum) -> Result<()> {
                self.0.push_value(value)
            }
        }

        impl DefaultArrayBuilder for Enum {
            type ArrayBuilder = Builder;

            fn default_builder() -> Self::ArrayBuilder {
                Builder(SparseUnionBuilder {
                    types: Vec::new(),
                    meta: vec![
                        FieldMeta {
                            name: String::from("A"),
                            ..Default::default()
                        },
                        FieldMeta {
                            name: String::from("B"),
                            ..Default::default()
                        },
                    ],
                    children: (
                        <i32 as DefaultArrayBuilder>::default_builder(),
                        <i64 as DefaultArrayBuilder>::default_builder(),
                    ),
                })
            }
        }
    };

    // the public API
    let mut builder = Enum::default_builder();
    builder.push_value(&Enum::A(13)).unwrap();
    builder.push_value(&Enum::B(21)).unwrap();
    let array = builder.build_array().unwrap();

    let [(_, _, a), (_, _, b)] = array.into_union_fields().expect("invalid array type");
    let a = a.into_int32().expect("invalid array type");
    let b = b.into_int64().expect("invalid array type");

    assert_eq!(a.values, vec![13, 0]);
    assert_eq!(b.values, vec![0, 21]);
}
