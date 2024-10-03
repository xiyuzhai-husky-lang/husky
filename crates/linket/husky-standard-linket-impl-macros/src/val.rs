// todo: move to ml-task-macros
use super::*;
use quote::quote;
use syn::parse::discouraged::AnyDelimiter;

pub(crate) fn val(args: TokenStream, input: TokenStream) -> TokenStream {
    let ValArgs {
        item_path_id_interface,
        var_deps,
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
    let expr_ty = if return_leash {
        match return_leash_ty {
            Some(return_leash_ty) => quote! { #return_leash_ty },
            None => quote! { Leash<#return_ty> },
        }
    } else {
        quote! {#return_ty}
    };
    if lazy {
        quote! {
            #vis fn #ident() -> #expr_ty {
                __eval_lazy_val(
                    unsafe { #item_path_id_interface.expect("ITEM_PATH_ID_INTERFACE not initialized") },
                    pedestal!(#var_deps),
                )
            }
        }
        .into()
    } else {
        quote! {
            #vis fn #ident() -> #expr_ty {
                __eval_eager_val_with(
                    unsafe { #item_path_id_interface.expect("ITEM_PATH_ID_INTERFACE not initialized") },
                    pedestal!(#var_deps),
                    || __KiControlFlow::Continue(#aux_ident().into_value())
                )
            }

            #vis fn #aux_ident() -> #return_ty #block
        }
        .into()
    }
}

struct ValArgs {
    item_path_id_interface: Ident,
    var_deps: syn::punctuated::Punctuated<syn::Path, syn::Token![,]>,
    // default false
    lazy: bool,
    // default false
    return_leash: bool,
    return_leash_ty: Option<syn::Type>,
}

impl syn::parse::Parse for ValArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = syn::Ident::parse_any(input)?;
        assert!(ident == "item_path_id_interface");
        let _eq = Equals::parse(input)?;
        let item_path_id_interface = syn::Ident::parse(input)?;
        let _comma = Comma::parse(input)?;
        let ident: syn::Ident = syn::Ident::parse_any(input)?;
        assert!(ident == "var_deps");
        let _eq = Equals::parse(input)?;
        let inside_brackets;
        let brackets = syn::bracketed!(inside_brackets in input);
        let var_deps = syn::punctuated::Punctuated::parse_terminated(&inside_brackets)?;
        let mut slf = Self {
            item_path_id_interface,
            var_deps,
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
