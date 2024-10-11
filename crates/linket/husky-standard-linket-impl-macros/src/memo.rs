use super::*;
use quote::quote;
use syn::{Ident, ItemFn, ReturnType, Signature};

// todo: allow customization on self value type
pub(crate) fn memo(args: TokenStream, input: TokenStream) -> TokenStream {
    let MemoizedFieldArgs {
        item_path_id_interface,
        return_leash,
        return_leash_ty,
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
    let expr_ty = if return_leash {
        match return_leash_ty {
            Some(return_leash_ty) => quote! { #return_leash_ty },
            None => quote! { Leash<#return_ty> },
        }
    } else {
        quote! {#return_ty}
    };
    quote! {
        #vis fn #ident(__self: Leash<Self>) -> #expr_ty {
            __eval_memo_field_with(
                unsafe { #item_path_id_interface.expect("ITEM_PATH_ID_INTERFACE not initialized") },
                __self.deleash(),
                |__self| {
                    // todo: catch unwind
                    __KiControlFlow::Continue(Self::#aux_ident(Leash(__self)).into_value())
                }
            )
        }

        #vis fn #aux_ident(__self: Leash<Self>) -> #return_ty #block
    }
    .into()
}

struct MemoizedFieldArgs {
    item_path_id_interface: Ident,
    // default false
    return_leash: bool,
    return_leash_ty: Option<syn::Type>,
}

impl syn::parse::Parse for MemoizedFieldArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = syn::Ident::parse_any(input)?;
        assert!(ident == "item_path_id_interface");
        let _eq = Equals::parse(input)?;
        let item_path_id_interface = syn::Ident::parse(input)?;
        let mut slf = Self {
            item_path_id_interface,
            return_leash: false,
            return_leash_ty: None,
        };
        loop {
            if input.is_empty() {
                return Ok(slf);
            }
            let _comma = Comma::parse(input)?;
            let ident: syn::Ident = syn::Ident::parse_any(input)?;
            if ident == "return_leash" {
                use syn::token::Token;

                assert!(!slf.return_leash);
                slf.return_leash = true;
                assert!(slf.return_leash_ty.is_none());
                if <syn::Token![=]>::peek(input.cursor()) {
                    let _: syn::Token![=] = input.parse()?;
                    slf.return_leash_ty = Some(input.parse()?);
                }
            } else {
                panic!()
            }
        }
    }
}
