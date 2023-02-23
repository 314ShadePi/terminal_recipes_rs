use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Ident};

#[allow(clippy::too_many_lines)]
pub fn commands_enum_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
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
        .iter()
        .map(|e| e.ident.to_string())
        .collect::<Vec<String>>();

    let variant_cmds = variants
        .iter()
        .map(|e| {
            let cmd_ident = Ident::new(e, Span::call_site());
            let cmd_c_ident = Ident::new("CMD", Span::call_site());
            let ret = quote! {#cmd_ident::#cmd_c_ident};
            ret
        })
        .collect::<Vec<_>>();

    let runners = variants
        .iter()
        .map(|e| {
            let cmd_ident = Ident::new(e, Span::call_site());
            let ret = quote! {Self::#cmd_ident(cmd) => cmd.run()};
            ret
        })
        .collect::<Vec<_>>();

    let variant_ctors = variants
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
                    )?,
                ))
            };
            ret
        })
        .collect::<Vec<_>>();

    let vhelp_short = variants.iter().map(|e| {
        let cmd_ident = Ident::new(e, Span::call_site());
        let cmd_h_ident = Ident::new("HELP_SHORT", Span::call_site());
        let vhelp_ident = quote! {#cmd_ident::#cmd_h_ident};
        let ret = quote! {println!("{}", #vhelp_ident);};
        ret
    }).collect::<Vec<_>>();
    let vhelp_long = variants.iter().map(|e| {
        let cmd_ident = Ident::new(e, Span::call_site());
        let cmd_h_ident = Ident::new("HELP_LONG", Span::call_site());
        let vhelp_ident = quote! {#cmd_ident::#cmd_h_ident};
        let cmd_c_ident = Ident::new("CMD", Span::call_site());
        let matcher = quote! {#cmd_ident::#cmd_c_ident};
        let ret = quote! {
            #matcher => {
                println!("{}", #vhelp_ident);
            }
        };
        ret
    }).collect::<Vec<_>>();

    Ok(quote! {
        use inquire::validator::Validation;
        use inquire::CustomUserError;
        use inquire::Text;
        use cmd_sys::Command;
        use anyhow::bail;
        impl #impl_generics EnumCommandLine for #name #ty_generics #where_clause {
            #[tracing::instrument]
            fn run(&self) -> anyhow::Result<()> {
                match self {
                    #(#runners),*
                }
            }

            #[tracing::instrument]
            fn validate(input: &str) -> Result<Validation, CustomUserError> {
                let s: (&str, &str) = match input.contains(" ") {
                    true => match input.split_once(' ') {
                        None => return Ok(Validation::Invalid("Couldn't parse command.".into())),
                        Some(s) => s,
                    },
                    false => (input, ""),
                };

                if !["exit", "help", #(#variant_cmds),*].contains(&s.0) {
                    return Ok(Validation::Invalid("Not a command.".into()));
                }

                Ok(Validation::Valid)
            }

            #[tracing::instrument(skip(error_handler))]
            fn command_line<F>(prompt: &str, error_handler: F)
            where
                F: Fn(anyhow::Error) -> Result<(), ()>
            {
                loop {
                    let cmd = Text::new(prompt)
                        .with_validator(Self::validate)
                        .prompt();

                    match cmd {
                        Ok(s) => {
                            if s == "exit".to_string() {
                                return;
                            } else {
                                let mut split = match s.contains(" ") {
                                    true => s.split(' ').collect(),
                                    false => vec![s.as_str()],
                                };

                                if split[0].clone() == "help".to_string() {
                                    split.remove(0);
                                    if split.is_empty() || split[0].clone() == "" {
                                        match Self::help(None) {
                                            Ok(()) => {}
                                            Err(e) => match error_handler(e) {
                                                Ok(()) => {}
                                                Err(()) => {
                                                    return;
                                                }
                                            },
                                        }
                                    } else {
                                        match Self::help(Some(split[0])) {
                                            Ok(()) => {}
                                            Err(e) => match error_handler(e) {
                                                Ok(()) => {}
                                                Err(()) => {
                                                    return;
                                                }
                                            },
                                        }
                                    }
                                } else {
                                    match Self::from_cl(&s) {
                                        Ok(c) => match c.run() {
                                            Ok(()) => {}
                                            Err(e) => match error_handler(e) {
                                                Ok(()) => {}
                                                Err(()) => {
                                                    return;
                                                }
                                            },
                                        },
                                        Err(e) => match error_handler(e) {
                                            Ok(()) => {}
                                            Err(()) => {
                                                return;
                                            }
                                        },
                                    }
                                }

                            }
                        }
                        Err(_) => {}
                    }
                }
            }

            #[tracing::instrument]
            fn from_cl(s: &str) -> anyhow::Result<Self> {
                let s = match s.contains(" ") {
                    true => match s.split_once(' ') {
                        None => bail!("split_once() operation failed."),
                        Some(s) => s,
                    },
                    false => (s, ""),
                };

                match s {
                    #(#variant_ctors),*,
                    _ => bail!("Not a command."),
                }
            }

            #[tracing::instrument]
            fn help(cmd: Option<&str>) -> anyhow::Result<()> {
                match cmd {
                    None => {
                        println!("Terminal Recipes RS - Help.");
                        println!("Basic usage:");
                        println!("<command> [options]");
                        println!("Commands:");
                        #(#vhelp_short)*
                    }
                    Some(cmd) => {
                        match cmd {
                            #(#vhelp_long),*
                            _ => bail!("Not a command.")
                        }
                    }
                }

                Ok(())
            }
        }
    })
}
