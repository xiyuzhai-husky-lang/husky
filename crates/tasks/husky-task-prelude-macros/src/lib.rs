mod value;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn value(args: TokenStream, input: TokenStream) -> TokenStream {
    value::value(args, input)
}
