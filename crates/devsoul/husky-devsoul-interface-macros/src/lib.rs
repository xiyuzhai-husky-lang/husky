mod memo;
mod utils;
mod val;
mod value;

use self::utils::*;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn value(args: TokenStream, input: TokenStream) -> TokenStream {
    value::value(args, input)
}

#[proc_macro_attribute]
pub fn val(args: TokenStream, input: TokenStream) -> TokenStream {
    val::val(args, input)
}

#[proc_macro_attribute]
pub fn memo(args: TokenStream, input: TokenStream) -> TokenStream {
    memo::memo(args, input)
}
