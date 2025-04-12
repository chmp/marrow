use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Attribute, Data, DataEnum, DataStruct, DeriveInput, Expr, Field, Fields, GenericParam, Ident,
    Lit, LitStr, Meta, Token, punctuated::Punctuated, spanned::Spanned,
};

#[proc_macro_derive(TypeInfo, attributes(marrow_type_info))]
pub fn derive_type_info(input: TokenStream) -> TokenStream {
    derive_type_info_impl(input.into()).into()
}

fn derive_type_info_impl(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
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

fn derive_for_struct(input: &DeriveInput, data: &DataStruct) -> proc_macro2::TokenStream {
    let name = &input.ident;

    let generics_decl = &input.generics;
    let generics_use = if !input.generics.params.is_empty() {
        let generics_use = input.generics.params.iter().map(|p| match p {
            GenericParam::Const(p) => p.ident.to_token_stream(),
            GenericParam::Lifetime(p) => p.lifetime.to_token_stream(),
            GenericParam::Type(_) => panic!(),
        });
        quote! {
            <#(#generics_use),*>
        }
    } else {
        quote! {}
    };

    let fields = get_fields(&data.fields);
    let body = match fields.as_slice() {
        [] => panic!(),
        [(NameSource::Index, _, field)] => {
            // TODO: ensure no args
            let field_ty = &field.ty;
            quote! { <#field_ty>::get_field(context) }
        }
        fields => {
            let mut field_exprs = Vec::new();

            for (_, field_name, field) in fields {
                let ty = &field.ty;
                let args = FieldArgs::from_attrs(&field.attrs);

                if let Some(func) = args.with.as_ref() {
                    field_exprs.push(quote! {
                        fields.push(context.nest(#field_name, #func::<#ty>)?);
                    });
                } else {
                    field_exprs.push(quote! {
                        fields.push(context.get_field::<#ty>(#field_name)?);
                    })
                }
            }

            quote! {
                let mut fields = ::std::vec::Vec::<::marrow::datatypes::Field>::new();
                    #( #field_exprs; )*

                    Ok(::marrow::datatypes::Field {
                        name: ::std::string::String::from(context.get_name()),
                        data_type: ::marrow::datatypes::DataType::Struct(fields),
                        nullable: false,
                        metadata: ::std::default::Default::default(),
                    })
            }
        }
    };

    quote! {
        const _: ()  = {
            impl #generics_decl ::marrow_convert::TypeInfo for #name #generics_use {
                fn get_field(
                    context: ::marrow_convert::Context<'_>,
                ) -> ::marrow_convert::Result<::marrow::datatypes::Field> {
                    #body
                }
            }
        };
    }
}

fn derive_for_enum(input: &DeriveInput, data: &DataEnum) -> proc_macro2::TokenStream {
    let mut variant_exprs = Vec::new();

    let name = &input.ident;
    let generics_decl = &input.generics;
    let generics_use = if !input.generics.params.is_empty() {
        let generics_use = input.generics.params.iter().map(|p| match p {
            GenericParam::Const(p) => p.ident.to_token_stream(),
            GenericParam::Lifetime(p) => p.lifetime.to_token_stream(),
            GenericParam::Type(_) => panic!(),
        });
        quote! {
            <#(#generics_use),*>
        }
    } else {
        quote! {}
    };

    for (idx, variant) in data.variants.iter().enumerate() {
        let variant_name = &variant.ident;
        let variant_name = LitStr::new(&variant_name.to_string(), variant_name.span());
        let variant_args = VariantArgs::from_attrs(&variant.attrs);

        if let Some(func) = variant_args.with.as_ref() {
            variant_exprs.push(quote! { #func(stringify!(#variant_name)) });
            continue;
        }

        let variant_idx = i8::try_from(idx).unwrap();

        let fields = get_fields(&variant.fields);
        match fields.as_slice() {
            [] => {
                // use nesting to allow overwrites
                variant_exprs.push(quote! {
                    (#variant_idx, context.nest(#variant_name, |context| {
                        Ok(::marrow::datatypes::Field {
                            name: ::std::string::String::from(context.get_name()),
                            data_type: ::marrow::datatypes::DataType::Null,
                            nullable: true,
                            metadata: ::std::default::Default::default(),
                        })
                    })?)
                });
            }
            [(NameSource::Index, _, field)] => {
                let field_ty = &field.ty;
                variant_exprs.push(quote! {
                    (#variant_idx, context.nest(#variant_name, <#field_ty>::get_field)?)
                });
            }
            fields => {
                let mut field_exprs = Vec::new();
                for (_, field_name, field) in fields {
                    let field_ty = &field.ty;
                    field_exprs.push(quote! {
                        context.get_field::<#field_ty>(#field_name)?
                    });
                }
                variant_exprs.push(quote! {
                    (#variant_idx, context.nest(#variant_name, |context| Ok(::marrow::datatypes::Field {
                        name: ::std::string::String::from(context.get_name()),
                        data_type: ::marrow::datatypes::DataType::Struct(vec![#(#field_exprs),*]),
                        nullable: false,
                        metadata: ::std::default::Default::default(),
                    }))?)
                });
            }
        }
    }

    quote! {
        const _: ()  = {
            impl #generics_decl ::marrow_convert::TypeInfo for #name #generics_use {
                fn get_field(
                    context: ::marrow_convert::Context<'_>,
                ) -> ::marrow_convert::Result<::marrow::datatypes::Field> {
                    let mut variants = ::std::vec::Vec::<(::std::primitive::i8, ::marrow::datatypes::Field)>::new();
                    #( variants.push(#variant_exprs); )*

                    Ok(::marrow::datatypes::Field {
                        name: ::std::string::String::from(context.get_name()),
                        data_type: ::marrow::datatypes::DataType::Union(variants, ::marrow::datatypes::UnionMode::Dense),
                        nullable: false,
                        metadata: ::std::default::Default::default(),
                    })
                }
            }
        };
    }
}

fn get_fields(fields: &Fields) -> Vec<(NameSource, LitStr, &Field)> {
    let mut result = Vec::new();
    match fields {
        Fields::Unit => {}
        Fields::Named(fields) => {
            for field in &fields.named {
                let Some(name) = field.ident.as_ref() else {
                    unreachable!("Named field must have a name");
                };
                let name = LitStr::new(&name.to_string(), name.span());
                result.push((NameSource::Ident, name, field));
            }
        }
        Fields::Unnamed(fields) => {
            for (idx, field) in fields.unnamed.iter().enumerate() {
                let name = LitStr::new(&idx.to_string(), field.span());
                result.push((NameSource::Index, name, field));
            }
        }
    }
    result
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum NameSource {
    Ident,
    Index,
}

#[test]
#[should_panic(expected = "Deriving TypeInfo for generics with type parameters is not supported")]
fn reject_unsupported() {
    derive_type_info_impl(quote! {
        struct Example<T> {
            field: T,
        }
    });
}

#[test]
fn lifetimes_are_supported() {
    derive_type_info_impl(quote! {
        struct Example<'a> {
            field: &'a i64,
        }
    });
}

#[test]
fn const_params_are_supported() {
    derive_type_info_impl(quote! {
        struct Example<const N: usize> {
            field: [u8; N],
        }
    });
}
