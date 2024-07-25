// todo: move to ml-task-macros
use super::*;
use quote::quote;
use syn::parse::discouraged::AnyDelimiter;

pub(crate) fn val(args: TokenStream, input: TokenStream) -> TokenStream {
    let ValArgs {
        ingredient_index,
        lazy,
        return_leash,
        return_leash_ty,
    } = syn::parse_macro_input!(args as ValArgs);
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
    let return_leash_ty = match return_leash_ty {
        Some(return_leash_ty) => quote! { #return_leash_ty },
        None => quote! { Leash<#return_ty> },
    };
    if lazy {
        if return_leash {
            quote! {
                #vis fn #ident() -> #return_leash_ty {
                    todo!("return leash for lazy val")
                    // __eval_lazy_val(#ingredient_index)
                }
            }
            .into()
        } else {
            quote! {
                #vis fn #ident() -> #return_ty {
                    todo!("return copied for lazy val")
                    // __eval_lazy_val(#ingredient_index)
                }
            }
            .into()
        }
    } else {
        if return_leash {
            quote! {
                #vis fn #ident() -> #return_leash_ty {
                    todo!("return leash for eager val, change the return type")
                    // __eval_eager_val_with(
                    //     #ingredient_index,
                    //     || __KiControlFlow::Continue(__ValueLeashTest(#aux_ident()).into_value())
                    // )
                }

                #vis fn #aux_ident() -> #return_ty #block
            }
            .into()
        } else {
            quote! {
                #vis fn #ident() -> #return_ty {
                    __eval_eager_val_with(
                        #ingredient_index,
                        || __KiControlFlow::Continue(__ValueLeashTest(#aux_ident()).into_value())
                    )
                }

                #vis fn #aux_ident() -> #return_ty #block
            }
            .into()
        }
    }
}

struct ValArgs {
    ingredient_index: usize,
    // default false
    lazy: bool,
    // default false
    return_leash: bool,
    return_leash_ty: Option<syn::Type>,
}

impl syn::parse::Parse for ValArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = syn::Ident::parse_any(input)?;
        assert!(ident == "ingredient_index");
        let _eq = Equals::parse(input)?;
        let lit = syn::LitInt::parse(input)?;
        let ingredient_index: usize = lit.base10_parse()?;
        let mut slf = Self {
            ingredient_index,
            lazy: false,
            return_leash: false,
            return_leash_ty: None,
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
            } else if ident == "return_leash" {
                use syn::token::Token;

                assert!(!slf.return_leash);
                slf.return_leash = true;
                assert!(slf.return_leash_ty.is_none());
                if <syn::Token![=]>::peek(input.cursor()) {
                    let _: syn::Token![=] = input.parse()?;
                    slf.return_leash_ty = Some(input.parse()?);
                }
            }
        }
    }
}
