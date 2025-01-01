mod floated;
mod note;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::*;

#[proc_macro_attribute]
pub fn floated(attr: TokenStream, item: TokenStream) -> TokenStream {
    floated::floated(attr, item)
}

#[proc_macro_attribute]
pub fn note(attr: TokenStream, item: TokenStream) -> TokenStream {
    note::note(attr, item)
}
