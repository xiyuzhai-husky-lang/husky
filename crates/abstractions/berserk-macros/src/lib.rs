mod berserk;
mod memo;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::*;

#[proc_macro_attribute]
pub fn berserk(attr: TokenStream, item: TokenStream) -> TokenStream {
    berserk::berserk(attr, item)
}

#[proc_macro_attribute]
pub fn memo(attr: TokenStream, item: TokenStream) -> TokenStream {
    memo::memo(attr, item)
}
