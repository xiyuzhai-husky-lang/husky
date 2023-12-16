mod user_ty;
mod value;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn value(args: TokenStream, input: TokenStream) -> TokenStream {
    value::value(args, input)
}

#[proc_macro_attribute]
pub fn user_ty(args: TokenStream, input: TokenStream) -> TokenStream {
    user_ty::user_ty(args, input)
}