// todo: move to ml-task-macros
use super::*;
use quote::quote;
use syn::{ext::IdentExt, Ident, ItemFn, ReturnType, Signature};

type Equals = syn::Token![=];
type Comma = syn::Token![,];

struct Args {
    ingredient_index: usize,
    // default false
    lazy: bool,
    // default false
    return_ref: bool,
}

impl syn::parse::Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = syn::Ident::parse_any(input)?;
        assert!(ident == "ingredient_index");
        let _eq = Equals::parse(input)?;
        let lit = syn::LitInt::parse(input)?;
        let ingredient_index: usize = lit.base10_parse()?;
        let mut slf = Self {
            ingredient_index,
            lazy: false,
            return_ref: false,
        };
        loop {
            if input.is_empty() {
                return Ok(slf);
            }
            let _comma = Comma::parse(input)?;
            let ident: syn::Ident = syn::Ident::parse_any(input)?;
            if ident == "lazy" {
                assert!(!slf.lazy);
                slf.lazy = true
            } else if ident == "return_ref" {
                assert!(!slf.return_ref);
                slf.return_ref = true
            }
        }
        Ok(slf)
    }
}

pub(crate) fn val_item(args: TokenStream, input: TokenStream) -> TokenStream {
    let Args {
        ingredient_index,
        lazy,
        return_ref,
    } = syn::parse_macro_input!(args as Args);
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
    if lazy {
        if return_ref {
            quote! {
                #vis fn #ident() -> &'static #return_ty {
                    __lazy_eval_val_item(#ingredient_index)
                }
            }
            .into()
        } else {
            quote! {
                #vis fn #ident() -> #return_ty {
                    __lazy_eval_val_item(#ingredient_index)
                }
            }
            .into()
        }
    } else {
        if return_ref {
            quote! {
                #vis fn #ident() -> &'static #return_ty {
                    __eager_eval_val_item(
                        #ingredient_index,
                        || {
                            #aux_ident();
                            todo!()
                        }
                    )
                }

                #vis fn #aux_ident() -> #return_ty #block
            }
            .into()
        } else {
            quote! {
                #vis fn #ident() -> #return_ty {
                    __eager_eval_val_item(
                        #ingredient_index,
                        || {
                            #aux_ident();
                            todo!()
                        }
                    )
                }

                #vis fn #aux_ident() -> #return_ty #block
            }
            .into()
        }
    }
}
