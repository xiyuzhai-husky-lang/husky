// todo: move to ml-task-macros
use super::*;
use quote::quote;

pub(crate) fn val_item(args: TokenStream, input: TokenStream) -> TokenStream {
    let ValItemArgs {
        ingredient_index,
        lazy,
        return_ref,
    } = syn::parse_macro_input!(args as ValItemArgs);
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
    if lazy {
        if return_ref {
            quote! {
                #vis fn #ident() -> &'static #return_ty {
                    __eval_lazy_val_item(#ingredient_index)
                }
            }
            .into()
        } else {
            quote! {
                #vis fn #ident() -> #return_ty {
                    __eval_lazy_val_item(#ingredient_index)
                }
            }
            .into()
        }
    } else {
        if return_ref {
            quote! {
                #vis fn #ident() -> &'static #return_ty {
                    __eval_eager_val_item_with(
                        #ingredient_index,
                        || __ValControlFlow::Continue(__ValueLeashTest(#aux_ident()).into_value())
                    )
                }

                #vis fn #aux_ident() -> #return_ty #block
            }
            .into()
        } else {
            quote! {
                #vis fn #ident() -> #return_ty {
                    __eval_eager_val_item_with(
                        #ingredient_index,
                        || __ValControlFlow::Continue(__ValueLeashTest(#aux_ident()).into_value())
                    )
                }

                #vis fn #aux_ident() -> #return_ty #block
            }
            .into()
        }
    }
}

struct ValItemArgs {
    ingredient_index: usize,
    // default false
    lazy: bool,
    // default false
    return_ref: bool,
}

impl syn::parse::Parse for ValItemArgs {
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
    }
}
