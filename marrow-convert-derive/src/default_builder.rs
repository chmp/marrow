use quote::{format_ident, quote};
use syn::{Data, DataEnum, DataStruct, DeriveInput, GenericParam, LitStr};

use super::array_push::get_fields_and_names;

pub fn derive_default_builder(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
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

    let builder_ident = format_ident!("{ident}Builder");

    let mut field_uses = Vec::new();
    let mut field_push = Vec::new();
    let mut field_builders = Vec::new();
    let mut field_metas = Vec::new();
    let mut field_inits = Vec::new();

    for (idx, (name, ty)) in get_fields_and_names(&data.fields).into_iter().enumerate() {
        let ident = format_ident!("t{idx}");
        field_uses.push(quote! { #ident });
        field_push.push(
            quote! { ::marrow_convert::builder::ArrayPush::push_value(#ident, &value.#name )?; },
        );

        let field_name = LitStr::new(&name.to_string(), name.span());

        field_builders
            .push(quote! { <#ty as ::marrow_convert::builder::DefaultArrayBuilder>::ArrayBuilder });
        field_metas.push(quote! {
            ::marrow::datatypes::FieldMeta {
                name: String::from(#field_name),
                ..::std::default::Default::default()
            }
        });
        field_inits.push(quote! {
            <#ty as ::marrow_convert::builder::DefaultArrayBuilder>::default_builder()
        })
    }

    return quote! {
        const _: () = {
            pub struct #builder_ident(::marrow_convert::builder::compound::StructBuilder<(#(#field_builders,)*)>);

            impl ::marrow_convert::builder::DefaultArrayBuilder for #ident {
                type ArrayBuilder = #builder_ident;

                fn default_builder() -> Self::ArrayBuilder {
                    #builder_ident(::marrow_convert::builder::compound::StructBuilder {
                        len: 0,
                        meta: vec![#(#field_metas),*],
                        children: (#(#field_inits,)*),
                    })
                }
            }

            impl ::marrow_convert::builder::ArrayBuilder for #builder_ident {
                fn push_default(&mut self) -> ::marrow_convert::Result<()> {
                    self.0.push_default()
                }

                fn build_array(&mut self) -> ::marrow_convert::Result<::marrow::array::Array> {
                    self.0.build_array()
                }
            }

            impl ::marrow_convert::builder::ArrayPush<#ident> for #builder_ident {
                fn push_value(&mut self, value: &#ident) -> ::marrow_convert::Result<()> {
                    self.0.len += 1;
                    let (#(#field_uses,)*) = &mut self.0.children;
                    #(#field_push)*
                    Ok(())
                }
            }

        };
    };
}

fn derive_for_enum(input: &DeriveInput, data: &DataEnum) -> proc_macro2::TokenStream {
    let _ = (input, data);
    todo!()
}
