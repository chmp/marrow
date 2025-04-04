use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, Data, DataEnum, DataStruct, DeriveInput, Expr, Field, Fields, Ident, Lit, LitStr,
    Meta, Token, parse_macro_input, punctuated::Punctuated, spanned::Spanned,
};

#[proc_macro_derive(TypeInfo, attributes(marrow_type_info))]
pub fn array_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if !input.generics.params.is_empty() {
        panic!("Deriving TypeInfo for generic is not supported")
    }

    let expanded = match input.data {
        Data::Struct(data) => derive_for_struct(&input.ident, &data),
        Data::Enum(data) => derive_for_enum(&input.ident, &data),
        Data::Union(_) => panic!("Deriving TypeInfo for unions is not supported"),
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

fn derive_for_struct(name: &Ident, data: &DataStruct) -> proc_macro2::TokenStream {
    let fields = get_fields(&data.fields);
    let body = match fields.as_slice() {
        [] => panic!(),
        [(NameSource::Index, _, field)] => {
            // TODO: ensure no args
            let field_ty = &field.ty;
            quote! { context.get_context().get_field::<#field_ty>(name) }
        }
        fields => {
            let mut field_exprs = Vec::new();

            for (_, field_name, field) in fields {
                let ty = &field.ty;
                let args = FieldArgs::from_attrs(&field.attrs);

                if let Some(func) = args.with.as_ref() {
                    field_exprs.push(quote! {
                        // TODO: pass context, include type?
                        fields.push(#func::<#ty>(context.get_context(), #field_name));
                    });
                } else {
                    field_exprs.push(quote! {
                        fields.push(context.get_context().get_field::<#ty>(#field_name)?);
                    })
                }
            }

            quote! {
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
                    #body
                }
            }
        };
    }
}

fn derive_for_enum(name: &Ident, data: &DataEnum) -> proc_macro2::TokenStream {
    let mut variant_exprs = Vec::new();

    for (idx, variant) in data.variants.iter().enumerate() {
        let variant_name = &variant.ident;
        let variant_args = VariantArgs::from_attrs(&variant.attrs);

        if let Some(func) = variant_args.with.as_ref() {
            variant_exprs.push(quote! { #func(stringify!(#variant_name)) });
            continue;
        }

        let variant_idx = i8::try_from(idx).unwrap();

        let fields = get_fields(&variant.fields);
        match fields.as_slice() {
            [] => {
                variant_exprs.push(quote! {
                    (#variant_idx, ::marrow::datatypes::Field {
                        name: ::std::string::String::from(stringify!(#variant_name)),
                        data_type: ::marrow::datatypes::DataType::Null,
                        nullable: true,
                        metadata: ::std::default::Default::default(),
                    })
                });
            }
            [(NameSource::Index, _, field)] => {
                let field_ty = &field.ty;
                variant_exprs.push(quote! {
                    (#variant_idx, context.get_context().get_field::<#field_ty>(stringify!(#variant_name))?)
                });
            }
            fields => {
                let mut field_exprs = Vec::new();
                for (_, field_name, field) in fields {
                    let field_ty = &field.ty;
                    field_exprs.push(quote! {
                        context.get_context().get_field::<#field_ty>(#field_name)?
                    });
                }
                variant_exprs.push(quote! {
                    (#variant_idx, ::marrow::datatypes::Field {
                        name: ::std::string::String::from(stringify!(#variant_name)),
                        data_type: ::marrow::datatypes::DataType::Struct(vec![#(#field_exprs),*]),
                        nullable: false,
                        metadata: ::std::default::Default::default(),
                    })
                });
            }
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
                    #( variants.push(#variant_exprs); )*

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
