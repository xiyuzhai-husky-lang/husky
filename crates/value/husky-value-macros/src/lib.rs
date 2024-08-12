mod value_ty;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn value_ty(args: TokenStream, input: TokenStream) -> TokenStream {
    value_ty::value_ty(args, input)
}
