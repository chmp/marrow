use marrow::datatypes::{DataType, Field};

use crate::{
    Result,
    types::{Context, DefaultArrayType},
};

use super::utils::new_list_field;

impl<T: DefaultArrayType> DefaultArrayType for [T] {
    fn get_field(context: Context<'_>) -> Result<Field> {
        new_list_field::<T>(context)
    }
}

impl<const N: usize, T: DefaultArrayType> DefaultArrayType for [T; N] {
    fn get_field(context: Context<'_>) -> Result<Field> {
        let base_field = context.get_field::<T>("element")?;
        let n = i32::try_from(N)?;

        // TODO: allow to customize
        let data_type = if matches!(base_field.data_type, DataType::UInt8) {
            DataType::FixedSizeBinary(n)
        } else {
            DataType::FixedSizeList(Box::new(base_field), n)
        };

        Ok(Field {
            name: context.get_name().to_owned(),
            data_type,
            nullable: false,
            metadata: Default::default(),
        })
    }
}

macro_rules! impl_tuples {
    ($( ( $($name:ident,)* ), )*) => {
        $(
            impl<$($name: DefaultArrayType),*> DefaultArrayType for ( $($name,)* ) {
                #[allow(unused_assignments, clippy::vec_init_then_push)]
                fn get_field(context: Context<'_>) -> Result<Field> {
                    let mut idx = 0;
                    let mut fields = Vec::new();
                    $(
                        fields.push(context.get_field::<$name>(&idx.to_string())?);
                        idx += 1;
                    )*

                    Ok(Field {
                        name: context.get_name().to_owned(),
                        data_type: DataType::Struct(fields),
                        ..Field::default()
                    })
                }
            }
        )*
    };
}

impl_tuples!(
    (A,),
    (A, B,),
    (A, B, C,),
    (A, B, C, D,),
    (A, B, C, D, E,),
    (A, B, C, D, E, F,),
    (A, B, C, D, E, F, G,),
    (A, B, C, D, E, F, G, H,),
    (A, B, C, D, E, F, G, H, I,),
    (A, B, C, D, E, F, G, H, I, J,),
    (A, B, C, D, E, F, G, H, I, J, K,),
    (A, B, C, D, E, F, G, H, I, J, K, L,),
    (A, B, C, D, E, F, G, H, I, J, K, L, M,),
    (A, B, C, D, E, F, G, H, I, J, K, L, M, N,),
    (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O,),
    (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,),
);
