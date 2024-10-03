use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(SettingSectionUi)]
pub fn derive_setting_section_ui(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl SettingSectionUi for #name {}
    };

    TokenStream::from(expanded)
}
