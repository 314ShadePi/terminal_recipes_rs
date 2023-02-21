use proc_macro2::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod config_options;

#[proc_macro_derive(ConfigOptions)]
pub fn config_options(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let tokens: TokenStream = config_options::config_options_inner(&ast).unwrap_or_else(|err| err.to_compile_error());
    tokens.into()
}