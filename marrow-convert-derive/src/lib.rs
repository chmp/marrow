use proc_macro::TokenStream;

mod array_push;
mod default_builder;
mod type_info;

#[proc_macro_derive(DefaultArrayType, attributes(marrow))]
pub fn derive_type_info(input: TokenStream) -> TokenStream {
    type_info::derive_type_info(input.into()).into()
}

#[proc_macro_derive(ArrayPush, attributes(marrow))]
pub fn derive_array_push(input: TokenStream) -> TokenStream {
    array_push::derive_array_push(input.into()).into()
}

#[proc_macro_derive(DefaultArrayBuilder, attributes(marrow))]
pub fn derive_default_builder(input: TokenStream) -> TokenStream {
    default_builder::derive_default_builder(input.into()).into()
}
