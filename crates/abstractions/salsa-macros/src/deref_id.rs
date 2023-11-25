use crate::options::Options;
use syn::Item;

pub(crate) fn deref_id(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // let options = syn::parse_macro_input!(args as Args);
    // let jar_ty = options.jar_ty();
    let item = syn::parse_macro_input!(input as Item);
    let Item::Struct(ref item) = item else {
        panic!("expect struct for `wrap_id`")
    };
    let ident = &item.ident;
    match item.fields {
        syn::Fields::Named(ref fields) => {
            if fields.named.len() != 1 {
                todo!()
            }
            let field_ident = &fields.named[0].ident;
            let field_ty = &fields.named[0].ty;
            let self_ty = if item.generics.params.is_empty() {
                quote! { #ident }
            } else {
                let arguments = syn::punctuated::Punctuated::<_, syn::Token![,]>::from_iter(
                    item.generics.params.iter().map(|param| match param {
                        syn::GenericParam::Type(param) => {
                            let ident = &param.ident;
                            quote! { #ident }
                        }
                        syn::GenericParam::Lifetime(param) => {
                            let lifetime = &param.lifetime;
                            quote! { #lifetime }
                        }
                        syn::GenericParam::Const(param) => {
                            let ident = &param.ident;
                            quote! { #ident }
                        }
                    }),
                );
                quote! { #ident<#arguments> }
            };
            let where_clause = &item.generics.where_clause;
            if where_clause.is_some() {
                todo!()
            }
            quote! {
                #item

                impl std::ops::Deref for #self_ty #where_clause {
                    type Target = #field_ty;

                    fn deref(&self) -> &#field_ty {
                        &self.#field_ident
                    }
                }
            }
            .into()
        }
        syn::Fields::Unnamed(ref fields) => {
            if fields.unnamed.len() != 1 {
                todo!()
            }
            let field_ty = &fields.unnamed[0].ty;
            let self_ty = if item.generics.params.is_empty() {
                quote! { #ident }
            } else {
                let arguments = syn::punctuated::Punctuated::<_, syn::Token![,]>::from_iter(
                    item.generics.params.iter().map(|param| match param {
                        syn::GenericParam::Type(param) => {
                            let ident = &param.ident;
                            quote! { #ident }
                        }
                        syn::GenericParam::Lifetime(param) => {
                            let lifetime = &param.lifetime;
                            quote! { #lifetime }
                        }
                        syn::GenericParam::Const(param) => {
                            let ident = &param.ident;
                            quote! { #ident }
                        }
                    }),
                );
                quote! { #ident<#arguments> }
            };
            let where_clause = &item.generics.where_clause;
            if where_clause.is_some() {
                todo!()
            }
            quote! {
                #item

                impl std::ops::Deref for #self_ty #where_clause {
                    type Target = #field_ty;

                    fn deref(&self) -> &#field_ty {
                        &self.0
                    }
                }
            }
            .into()
        }
        syn::Fields::Unit => todo!(),
    }
}
