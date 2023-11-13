use crate::options::Options;

use syn::{Item};


type Args = Options<WrapId>;

struct WrapId;

impl crate::options::AllowedOptions for WrapId {
    const RETURN_REF: bool = false;

    const SPECIFY: bool = false;

    const NO_EQ: bool = false;

    const SINGLETON: bool = false;

    const JAR: bool = true;

    const DATA: bool = false;

    const DB: bool = false;

    const RECOVERY_FN: bool = false;

    const LRU: bool = false;

    const CONSTRUCTOR: bool = false;

    const OVERRIDE_DEBUG: bool = false;
}

pub(crate) fn wrap_id(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let options = syn::parse_macro_input!(args as Args);
    let jar_ty = options.jar_ty();
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

                impl ::salsa::AsId for #self_ty #where_clause {
                    fn as_id(self) -> salsa::Id {
                        self.#field_ident.as_id()
                    }

                    fn from_id(id: salsa::Id) -> Self {
                        #ident {
                            #field_ident: #field_ty::from_id(id)
                        }
                    }
                }

                impl<DB> salsa::salsa_struct::SalsaStructInDb<DB> for #self_ty
                where
                    DB: ?Sized + salsa::DbWithJar<#jar_ty>,
                {
                    fn register_dependent_fn(_db: &DB, _index: salsa::routes::IngredientIndex) {}
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

                impl ::salsa::AsId for #self_ty #where_clause {
                    fn as_id(self) -> salsa::Id {
                        self.0.as_id()
                    }

                    fn from_id(id: salsa::Id) -> Self {
                        #ident(#field_ty::from_id(id))
                    }
                }

                impl<DB> salsa::salsa_struct::SalsaStructInDb<DB> for #self_ty
                where
                    DB: ?Sized + salsa::DbWithJar<#jar_ty>,
                {
                    fn register_dependent_fn(_db: &DB, _index: salsa::routes::IngredientIndex) {}
                }
            }
            .into()
        }
        syn::Fields::Unit => todo!(),
    }
}
