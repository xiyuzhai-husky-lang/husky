mod stashes;
mod unify_elabm;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_attribute]
pub fn stashes(attr: TokenStream, input: TokenStream) -> TokenStream {
    stashes::stashes(attr, input)
}

#[proc_macro_attribute]
pub fn unify_elabm(attr: TokenStream, input: TokenStream) -> TokenStream {
    unify_elabm::unify_elabm(attr, input)
}
