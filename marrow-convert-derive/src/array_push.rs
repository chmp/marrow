use quote::{format_ident, quote};
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, Fields, GenericParam, Ident, Type, spanned::Spanned,
};

pub fn derive_array_push(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let input: DeriveInput = syn::parse2(input).unwrap();

    if input
        .generics
        .params
        .iter()
        .any(|p| matches!(p, GenericParam::Type(_)))
    {
        panic!("Deriving TypeInfo for generics with type parameters is not supported")
    }

    match &input.data {
        Data::Struct(data) => derive_for_struct(&input, data),
        Data::Enum(data) => derive_for_enum(&input, data),
        Data::Union(_) => panic!("Deriving TypeInfo for unions is not supported"),
    }
}

fn derive_for_struct(input: &DeriveInput, data: &DataStruct) -> proc_macro2::TokenStream {
    if data.fields.len() >= 16 {
        panic!("Only structs with at most 16 fields are supported");
    }

    let ident = &input.ident;
    let generics = &input.generics;
    let generics = if generics.params.is_empty() {
        quote! {}
    } else {
        quote! { #generics, }
    };

    let mut field_defs = Vec::new();
    let mut field_uses = Vec::new();
    let mut field_push = Vec::new();

    for (idx, (name, ty)) in get_fields_and_names(&data.fields).into_iter().enumerate() {
        let ident = format_ident!("T{idx}");
        field_defs.push(quote! { #ident : ::marrow_convert::builder::ArrayPush<#ty> });
        field_uses.push(quote! { #ident });
        field_push.push(
            quote! { ::marrow_convert::builder::ArrayPush::push_value(#ident, &value.#name )?; },
        );
    }

    quote! {
        const _: () = {
            impl<#generics #(#field_defs),*> ::marrow_convert::builder::ArrayPush<#ident> for ::marrow_convert::builder::compound::StructBuilder<(#(#field_uses,)*)> {
                fn push_value(&mut self, value: &#ident) -> ::marrow_convert::Result<()> {
                    self.len += 1;
                    let (#(#field_uses,)*) = &mut self.children;
                    #(#field_push)*
                    Ok(())
                }
            }
        };
    }
}

pub fn get_fields_and_names(fields: &Fields) -> Vec<(Ident, Type)> {
    let mut result = Vec::new();
    match fields {
        Fields::Named(fields) => {
            for field in &fields.named {
                let ident = field.ident.clone().expect("Named field without ident");
                let ty = field.ty.clone();
                result.push((ident, ty));
            }
        }
        Fields::Unnamed(fields) => {
            for (idx, field) in fields.unnamed.iter().enumerate() {
                result.push((
                    Ident::new(&idx.to_string(), field.ty.span()),
                    field.ty.clone(),
                ));
            }
        }
        Fields::Unit => unimplemented!("Unit structs are currently not implemented"),
    }

    result
}

fn derive_for_enum(input: &DeriveInput, data: &DataEnum) -> proc_macro2::TokenStream {
    let _ = (input, data);
    todo!()
}
