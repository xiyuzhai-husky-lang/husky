//! Proc-macros used in [Sycamore](https://sycamore-rs.netlify.app).

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod component;
mod prop;
mod view;

#[proc_macro]
pub fn view(view: TokenStream) -> TokenStream {
    let view_root = parse_macro_input!(view as view::WithcxArg<view::ir::ViewRoot>);

    view::view_impl(view_root).into()
}

#[proc_macro]
pub(crate) fn syn_node(input: TokenStream) -> TokenStream {
    let elem = parse_macro_input!(input as view::WithcxArg<view::ir::Element>);

    view::node_impl(elem).into()
}

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, component: TokenStream) -> TokenStream {
    let comp = {
        let component = component.clone();
        parse_macro_input!(component as component::ComponentFunction)
    };

    component::component_impl(comp)
        .unwrap_or_else(|err| {
            // If proc-macro errors, emit the original function for better IDE support.
            let error_tokens = err.into_compile_error();
            let component_tokens = proc_macro2::TokenStream::from(component);
            quote! {
                #component_tokens
                #error_tokens
            }
        })
        .into()
}

#[proc_macro_derive(Prop, attributes(builder))]
pub fn derive_prop(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    prop::impl_derive_prop(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
