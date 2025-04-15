pub trait TupleLen {
    const LEN: usize;
}

macro_rules! impl_tuple_len {
    ($head:ident, $($tail:ident,)*) => {
        impl<$head, $($tail),*> TupleLen for ($head, $($tail,)*) {
            const LEN: usize = 1 + <($($tail,)*) as TupleLen>::LEN;
        }
    };
    () => {
        impl TupleLen for () {
            const LEN: usize = 0;
        }
    };
}

impl_tuple_len!();
impl_tuple_len!(A,);
impl_tuple_len!(A, B,);
impl_tuple_len!(A, B, C,);
impl_tuple_len!(A, B, C, D,);
impl_tuple_len!(A, B, C, D, E,);
impl_tuple_len!(A, B, C, D, E, F,);
impl_tuple_len!(A, B, C, D, E, F, G,);
impl_tuple_len!(A, B, C, D, E, F, G, H,);
impl_tuple_len!(A, B, C, D, E, F, G, H, I,);
impl_tuple_len!(A, B, C, D, E, F, G, H, I, J,);
impl_tuple_len!(A, B, C, D, E, F, G, H, I, J, K,);
impl_tuple_len!(A, B, C, D, E, F, G, H, I, J, K, L,);
impl_tuple_len!(A, B, C, D, E, F, G, H, I, J, K, L, M,);
impl_tuple_len!(A, B, C, D, E, F, G, H, I, J, K, L, M, N,);
impl_tuple_len!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O,);
impl_tuple_len!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,);
