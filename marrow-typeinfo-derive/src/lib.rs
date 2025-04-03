use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, Data, DeriveInput, Expr, Fields, FieldsNamed, Ident, Lit, Meta, Token, Variant,
    parse_macro_input, punctuated::Punctuated, token::Comma,
};

#[proc_macro_derive(TypeInfo, attributes(marrow_type_info))]
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

#[derive(Debug, Default)]
struct FieldArgs {
    // TODO: use a path here
    with: Option<Ident>,
}

impl FieldArgs {
    pub fn from_attrs(attrs: &[Attribute]) -> Self {
        let mut result = Self::default();

        for attr in attrs {
            if !attr.path().is_ident("marrow_type_info") {
                continue;
            }

            let nested = attr
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .unwrap();
            for meta in nested {
                match meta {
                    Meta::NameValue(meta) => {
                        if !meta.path.is_ident("with") {
                            continue;
                        }
                        match meta.value {
                            Expr::Lit(lit) => match lit.lit {
                                Lit::Str(str) => {
                                    result.with = Some(Ident::new(&str.value(), str.span()));
                                }
                                _ => unimplemented!(),
                            },
                            _ => unimplemented!(),
                        }
                    }
                    _ => unimplemented!(),
                }
            }
        }
        result
    }
}

#[derive(Debug, Default)]
struct VariantArgs {
    with: Option<Ident>,
}

impl VariantArgs {
    pub fn from_attrs(attrs: &[Attribute]) -> Self {
        let mut result = Self::default();

        for attr in attrs {
            if !attr.path().is_ident("marrow_type_info") {
                continue;
            }

            let nested = attr
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .unwrap();
            for meta in nested {
                match meta {
                    Meta::NameValue(meta) => {
                        if !meta.path.is_ident("with") {
                            continue;
                        }
                        match meta.value {
                            Expr::Lit(lit) => match lit.lit {
                                Lit::Str(str) => {
                                    result.with = Some(Ident::new(&str.value(), str.span()));
                                }
                                _ => unimplemented!(),
                            },
                            _ => unimplemented!(),
                        }
                    }
                    _ => unimplemented!(),
                }
            }
        }
        result
    }
}

fn derive_for_struct(name: &Ident, fields: &FieldsNamed) -> proc_macro2::TokenStream {
    let mut field_exprs = Vec::new();

    for field in &fields.named {
        let field_name = field.ident.as_ref().expect("named filed without ident");
        let ty = &field.ty;
        let args = FieldArgs::from_attrs(&field.attrs);

        if let Some(func) = args.with.as_ref() {
            field_exprs.push(quote! {
                // TODO: pass context, include type?
                fields.push(#func(context.get_context(), stringify!(#field_name)));
            });
        } else {
            field_exprs.push(quote! {
                fields.push(context.get_context().get_field::<#ty>(stringify!(#field_name))?);
            })
        }
    }

    quote! {
        const _: ()  = {
            impl ::marrow_typeinfo::TypeInfo for #name {
                fn get_field(
                    name: &::std::primitive::str,
                    context: ::marrow_typeinfo::ContextRef<'_>,
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
        let variant_args = VariantArgs::from_attrs(&variant.attrs);

        if let Some(func) = variant_args.with.as_ref() {
            variant_exprs.push(quote! { #func(stringify!(#variant_name)) });
            continue;
        }

        match &variant.fields {
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
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                let Some(field) = fields.unnamed.first() else {
                    unreachable!("checked in guard that exactly 1 field is available");
                };

                let field_ty = &field.ty;
                variant_exprs.push(quote! {
                    variants.push((i8::try_from(#idx)?, context.get_context().get_field::<#field_ty>(stringify!(#variant_name))?));
                });
            }
            Fields::Unnamed(_) => panic!("enums with unnamed fields are currently supported"),
            Fields::Named(_) => panic!("enums with named fields are currently supported"),
        }
    }

    quote! {
        const _: ()  = {
            impl ::marrow_typeinfo::TypeInfo for #name {
                fn get_field(
                    name: &::std::primitive::str,
                    context: ::marrow_typeinfo::ContextRef<'_>,
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
