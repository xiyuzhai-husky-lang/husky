use super::*;
use quote::quote;
use syn::{Ident, ItemFn, ReturnType, Signature};

pub(crate) fn memoized_field(args: TokenStream, input: TokenStream) -> TokenStream {
    let MemoizedFieldArgs {
        ingredient_index,
        return_ref,
    } = syn::parse_macro_input!(args as MemoizedFieldArgs);
    let ItemFn {
        attrs: _,
        vis,
        sig:
            Signature {
                constness: _,
                asyncness: _,
                unsafety: _,
                abi: _,
                fn_token: _,
                ident,
                generics: _,
                paren_token: _,
                inputs: _,
                variadic: _,
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
                __eval_memoized_field_return_ref_with(
                    self,
                    #ingredient_index,
                    |slf| {
                        // todo: catch unwind
                        __ValControlFlow::Continue(__ValueLeashTest(slf.#aux_ident()).into_value())
                    }
                )
            }

            #vis fn #aux_ident(&'static self) -> #return_ty #block
        }
        .into()
    } else {
        quote! {
            #vis fn #ident(&'static self) -> #return_ty {
                __eval_memoized_field_with(
                    self,
                    #ingredient_index,
                    |slf| {
                        // todo: catch unwind
                        __ValControlFlow::Continue(__ValueLeashTest(slf.#aux_ident()).into_value())
                    }
                )
            }

            #vis fn #aux_ident(&'static self) -> #return_ty #block
        }
        .into()
    }
}

struct MemoizedFieldArgs {
    ingredient_index: usize,
    // default false
    return_ref: bool,
}

impl syn::parse::Parse for MemoizedFieldArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = syn::Ident::parse_any(input)?;
        assert!(ident == "ingredient_index");
        let _eq = Equals::parse(input)?;
        let lit = syn::LitInt::parse(input)?;
        let ingredient_index: usize = lit.base10_parse()?;
        let mut slf = Self {
            ingredient_index,
            return_ref: false,
        };
        loop {
            if input.is_empty() {
                return Ok(slf);
            }
            let _comma = Comma::parse(input)?;
            let ident: syn::Ident = syn::Ident::parse_any(input)?;
            if ident == "return_ref" {
                assert!(!slf.return_ref);
                slf.return_ref = true
            } else {
                panic!()
            }
        }
        Ok(slf)
    }
}
