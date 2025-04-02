use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Data, DeriveInput, Fields, FieldsNamed, Ident, Variant, parse_macro_input,
    punctuated::Punctuated, token::Comma,
};

#[proc_macro_derive(TypeInfo)]
pub fn array_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let expanded = match input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => derive_for_struct(&input.ident, fields),
            Fields::Unnamed(_) => {
                panic!("Deriving TypeInfo for tuple structs is not yet supported")
            }
            Fields::Unit => {
                panic!("Deriving TypeInfo for unit structs is not yet supported")
            }
        },
        Data::Enum(data) => derive_for_enum(&input.ident, &data.variants),
        Data::Union(_) => {
            panic!("Deriving TypeInfo for unions is currently not supported")
        }
    };

    TokenStream::from(expanded)
}

fn derive_for_struct(name: &Ident, fields: &FieldsNamed) -> proc_macro2::TokenStream {
    let mut field_exprs = Vec::new();

    for field in &fields.named {
        let field_name = field.ident.as_ref().expect("named filed without ident");
        let ty = &field.ty;

        field_exprs.push(quote! {
            fields.push(<#ty as ::marrow_typeinfo::TypeInfo>::get_field(stringify!(#field_name), context)?);
        })
    }

    quote! {
        const _: ()  = {
            impl ::marrow_typeinfo::TypeInfo for #name {
                fn get_field(
                    name: &::std::primitive::str,
                    context: &::marrow_typeinfo::Context,
                ) -> ::std::result::Result<
                    ::marrow::datatypes::Field,
                    ::marrow_typeinfo::Error,
                > {
                    let mut fields = ::std::vec::Vec::<::marrow::datatypes::Field>::new();
                    #( #field_exprs; )*

                    Ok(::marrow::datatypes::Field {
                        name: ::std::string::String::from(name),
                        data_type: ::marrow::datatypes::DataType::Struct(fields),
                        nullable: false,
                        metadata: ::std::default::Default::default(),
                    })
                }
            }
        };
    }
}

fn derive_for_enum(
    name: &Ident,
    variants: &Punctuated<Variant, Comma>,
) -> proc_macro2::TokenStream {
    let mut variant_exprs = Vec::new();

    for (idx, variant) in variants.iter().enumerate() {
        let variant_name = &variant.ident;

        match variant.fields {
            Fields::Unit => {
                variant_exprs.push(quote! {
                    variants.push((i8::try_from(#idx)?, ::marrow::datatypes::Field {
                        name: ::std::string::String::from(stringify!(#variant_name)),
                        data_type: ::marrow::datatypes::DataType::Null,
                        nullable: true,
                        metadata: ::std::default::Default::default(),
                    }));
                });
            }
            Fields::Named(_) => panic!("enums with named fields are currently supported"),
            Fields::Unnamed(_) => panic!("enums with unnamed fields are currently supported"),
        }
    }

    quote! {
        const _: ()  = {
            impl ::marrow_typeinfo::TypeInfo for #name {
                fn get_field(
                    name: &::std::primitive::str,
                    context: &::marrow_typeinfo::Context,
                ) -> ::std::result::Result<
                    ::marrow::datatypes::Field,
                    ::marrow_typeinfo::Error,
                > {
                    let mut variants = ::std::vec::Vec::<(::std::primitive::i8, ::marrow::datatypes::Field)>::new();
                    #( #variant_exprs; )*

                    Ok(::marrow::datatypes::Field {
                        name: ::std::string::String::from(name),
                        data_type: ::marrow::datatypes::DataType::Union(variants, ::marrow::datatypes::UnionMode::Dense),
                        nullable: false,
                        metadata: ::std::default::Default::default(),
                    })
                }
            }
        };
    }
}
