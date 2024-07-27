mod memo;
mod utils;
mod val;
mod value_ty;

use self::utils::*;
use proc_macro::TokenStream;

/// todo: move this somewhere else, say, husky-value-macros
#[proc_macro_attribute]
pub fn value_ty(args: TokenStream, input: TokenStream) -> TokenStream {
    value_ty::value_ty(args, input)
}

#[proc_macro_attribute]
pub fn val(args: TokenStream, input: TokenStream) -> TokenStream {
    val::val(args, input)
}

#[proc_macro_attribute]
pub fn memo(args: TokenStream, input: TokenStream) -> TokenStream {
    memo::memo(args, input)
}
