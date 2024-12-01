mod eterned;
mod memo;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::*;

#[proc_macro_attribute]
pub fn eterned(attr: TokenStream, item: TokenStream) -> TokenStream {
    eterned::eterned(attr, item)
}

#[proc_macro_attribute]
pub fn memo(attr: TokenStream, item: TokenStream) -> TokenStream {
    memo::memo(attr, item)
}
