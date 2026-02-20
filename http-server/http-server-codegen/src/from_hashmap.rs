use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Data, Fields, PathArguments, Type};

pub fn from_hashmap_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_from_hashmap(&ast)
}

fn impl_from_hashmap(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = match &ast.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("FromHashMap only works on structs with named fields."),
        },
        _ => panic!("FromHashMap can only be derived for structs."),
    };

    let initializers = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_name_str = field_name.to_string();
        let field_type = &field.ty;

        if let Some(inner_type) = get_option_inner_type(field_type) {
            let inner_type_str = quote!(#inner_type).to_string();

            let parser = if inner_type_str == "String" {
                quote! { map.get(#field_name_str).cloned() }
            } else {
                quote! {
                    map.get(#field_name_str)
                        .map(|s| s.parse::<#inner_type>())
                        .transpose()?
                }
            };
            quote! { #field_name: #parser }
        } else {
            let field_type_str = quote!(#field_type).to_string();

            let parser = if field_type_str == "String" {
                quote! {
                    map.get(#field_name_str)
                       .ok_or_else(|| format!("Missing required field: '{}'", #field_name_str))?
                       .clone()
                }
            } else {
                quote! {
                    map.get(#field_name_str)
                       .ok_or_else(|| format!("Missing required field: '{}'", #field_name_str))?
                       .parse::<#field_type>()?
                }
            };
            quote! { #field_name: #parser }
        }
    });

    let expanded = quote! {
        use std::collections::HashMap;
        use std::str::FromStr;
        use http_server_types::result::Result;

        impl #name {
            pub fn from_hashmap(map: &HashMap<String, String>) -> Result<Self> {
                Ok(Self {
                    #(#initializers),*
                })
            }
        }
    };

    TokenStream::from(expanded)
}

fn get_option_inner_type(ty: &Type) -> Option<&Type> {
    if let Type::Path(type_path) = ty
        && type_path.path.segments.len() == 1
    {
        let segment = &type_path.path.segments[0];
        if segment.ident == "Option"
            && let PathArguments::AngleBracketed(args) = &segment.arguments
            && args.args.len() == 1
            && let syn::GenericArgument::Type(inner_ty) = &args.args[0]
        {
            return Some(inner_ty);
        }
    }
    None
}
