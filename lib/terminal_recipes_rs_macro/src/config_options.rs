use convert_case::{Case, Casing};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Ident};

#[allow(clippy::too_many_lines)]
pub fn config_options_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let gen = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();
    let fields = match &ast.data {
        Data::Struct(f) => &f.fields,
        _ => {
            return Err(syn::Error::new(
                Span::call_site(),
                "This macro only supports structs.",
            ))
        }
    };

    let fields_pascal = fields.iter().map(|field| {
        field.ident.clone().unwrap().to_string().to_case(Case::Pascal)
    }).collect::<Vec<_>>();

    let fields_pascal_ident = fields_pascal.iter().map(|field| {
        let field_ident = Ident::new(field, Span::call_site());
        let ret = quote! {#field_ident};
        ret
    }).collect::<Vec<_>>();

    let enum_name = name.to_string() + "Opt";
    let enum_name = Ident::new(&enum_name, Span::call_site());

    Ok(quote! {
        use convert_case::{Case, Casing};
        use strum::{EnumIter, EnumString, EnumVariantNames, VariantNames};
        impl #impl_generics ConfigOptions for #name #ty_generics #where_clause {
            fn get_opts() -> Vec<String> {
                #enum_name::VARIANTS.iter().map(|v| {
                    v.to_case(Case::Kebab)
                }).collect::<Vec<_>>()
            }
        }
        #[derive(EnumIter, EnumString, EnumVariantNames)]
        pub enum #enum_name {
            #(#fields_pascal_ident),*,
        }
    })
}