use quote::quote;

pub(crate) fn user_ty(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = syn::parse_macro_input!(input as syn::Item);
    quote! {
        #item
    }
    .into()
}
