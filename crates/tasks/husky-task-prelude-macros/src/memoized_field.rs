use super::*;
use quote::quote;
use syn::{Ident, ItemFn, ReturnType, Signature};

pub(crate) fn memoized_field_aux(input: TokenStream, return_ref: bool) -> TokenStream {
    let ItemFn {
        attrs,
        vis,
        sig:
            Signature {
                constness,
                asyncness,
                unsafety,
                abi,
                fn_token,
                ident,
                generics,
                paren_token,
                inputs,
                variadic,
                output,
            },
        block,
    } = syn::parse_macro_input!(input as syn::ItemFn);
    let ReturnType::Type(_, ref return_ty) = output else {
        unreachable!()
    };
    let aux_ident = Ident::new(&format!("__{}", ident), ident.span());
    if return_ref {
        quote! {
            #vis fn #ident(&'static self) -> &'static #return_ty {
                todo!("memoized_field return_ref")
            }

            #vis fn #aux_ident(&'static self) -> #return_ty #block
        }
        .into()
    } else {
        quote! {
            #vis fn #ident(&'static self) -> #return_ty {
                todo!("memoized_field return")
            }

            #vis fn #aux_ident(&'static self) -> #return_ty #block
        }
        .into()
    }
}
