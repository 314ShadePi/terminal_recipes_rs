mod enum_command_line;

use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(EnumCommandLine)]
pub fn commands_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let tokens =
        enum_command_line::commands_enum_inner(&ast).unwrap_or_else(|err| err.to_compile_error());
    tokens.into()
}
