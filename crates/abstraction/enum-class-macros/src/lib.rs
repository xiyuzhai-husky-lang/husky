#![recursion_limit = "256"]

#[macro_use]
extern crate quote;

mod from_variants;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn from_variants(args: TokenStream, input: TokenStream) -> TokenStream {
    from_variants::from_variants(args, input)
}
