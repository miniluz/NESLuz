use proc_macro::{self, TokenStream};
use syn::{parse_macro_input, DeriveInput};

mod addressing_enum;

#[proc_macro_derive(AddressingEnum, attributes(modes))]
pub fn derive_addressing_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    addressing_enum::derive(input)
        .unwrap_or_else(|err| err.into_compile_error())
        .into()
}
