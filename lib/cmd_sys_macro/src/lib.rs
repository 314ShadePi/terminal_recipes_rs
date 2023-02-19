use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Ident};

#[proc_macro_derive(EnumCommandLine)]
pub fn commands_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let tokens = commands_enum_inner(&ast).unwrap_or_else(|err| err.to_compile_error());
    tokens.into()
}

fn commands_enum_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let gen = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();
    let variants = match &ast.data {
        Data::Enum(v) => &v.variants,
        _ => {
            return Err(syn::Error::new(
                Span::call_site(),
                "This macro only supports enums.",
            ))
        }
    };

    let variants = variants
        .clone()
        .iter()
        .map(|e| e.ident.to_string())
        .collect::<Vec<String>>();

    let variant_cmds = variants
        .clone()
        .iter()
        .map(|e| {
            let cmd_ident = Ident::new(e, Span::call_site());
            let cmd_c_ident = Ident::new("CMD", Span::call_site());
            let ret = quote! {#cmd_ident::#cmd_c_ident};
            ret
        })
        .collect::<Vec<_>>();

    let runners = variants
        .clone()
        .iter()
        .map(|e| {
            let cmd_ident = Ident::new(e, Span::call_site());
            let ret = quote! {Self::#cmd_ident(cmd) => cmd.run()};
            ret
        })
        .collect::<Vec<_>>();

    let variant_ctors = variants
        .clone()
        .iter()
        .map(|e| {
            let cmd_ident = Ident::new(e, Span::call_site());
            let cmd_c_ident = Ident::new("CMD", Span::call_site());
            let matcher = quote! {#cmd_ident::#cmd_c_ident};
            let ret = quote! {
                (#matcher, params) => Ok(Self::#cmd_ident(
                    #cmd_ident::new_cmd(
                        params
                            .split(' ')
                            .collect::<Vec<&str>>()
                            .iter()
                            .map(|e| e.to_string())
                            .collect(),
                    )
                    .unwrap(),
                ))
            };
            ret
        })
        .collect::<Vec<_>>();

    Ok(quote! {
        use inquire::validator::Validation;
        use inquire::CustomUserError;
        use inquire::Text;
        use cmd_sys::Command;
        impl #impl_generics EnumCommandLine for #name #ty_generics #where_clause {
            type Err = ();

            fn run(&self) -> Result<(), Self::Err> {
                match self {
                    #(#runners),*
                }
            }

            fn validate(input: &str) -> Result<Validation, CustomUserError> {
                let s: (&str, &str) = match input.contains(" ") {
                    true => match input.split_once(' ') {
                        None => return Ok(Validation::Invalid("Couldn't parse command.".into())),
                        Some(s) => s,
                    },
                    false => (input, ""),
                };

                if !["exit", #(#variant_cmds),*].contains(&s.0) {
                    return Ok(Validation::Invalid("Not a command.".into()));
                }

                Ok(Validation::Valid)
            }

            fn command_line(prompt: &str) {
                loop {
                    let cmd = Text::new(prompt)
                        .with_validator(Self::validate)
                        .prompt();

                    match cmd {
                        Ok(s) => {
                            if s == "exit".to_string() {
                                return;
                            } else {
                                Self::from_cl(&s).unwrap().run().unwrap()
                            }
                        }
                        Err(_) => {}
                    }
                }
            }

            fn from_cl(s: &str) -> Result<Self, Self::Err> {
                let s = match s.contains(" ") {
                    true => match s.split_once(' ') {
                        None => return Err(()),
                        Some(s) => s,
                    },
                    false => (s, ""),
                };

                match s {
                    #(#variant_ctors),*,
                    _ => Err(()),
                }
            }
        }
    })
}
