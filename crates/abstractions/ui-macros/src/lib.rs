mod settings;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(SettingSectionUi)]
pub fn derive_setting_section_ui(input: TokenStream) -> TokenStream {
    settings::section::derive_setting_section_ui(input)
}
