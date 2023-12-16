mod conversion;

use proc_macro::TokenStream;

/// implement From<Value> and To<Value>
#[proc_macro_attribute]
pub fn value_conversion(args: TokenStream, input: TokenStream) -> TokenStream {
    conversion::value_conversion(args, input)
}
