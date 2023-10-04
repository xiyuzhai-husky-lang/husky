use syn::{ItemStruct, Path};

pub(super) fn generic_decls(generics: &syn::Generics, db_path: &Path) -> proc_macro2::TokenStream {
    if generics.params.is_empty() {
        quote! { _Db:  #db_path + ?Sized }
    } else {
        let lifetime_generics_punctuated =
            syn::punctuated::Punctuated::<_, syn::Token![,]>::from_iter(
                generics.params.iter().filter_map(|param| match param {
                    syn::GenericParam::Type(_) | syn::GenericParam::Const(_) => None,
                    syn::GenericParam::Lifetime(_) => Some(quote! { #param }),
                }),
            );
        let nonlifetime_generics_punctuated =
            syn::punctuated::Punctuated::<_, syn::Token![,]>::from_iter(
                generics.params.iter().filter_map(|param| match param {
                    syn::GenericParam::Type(param) => Some(if param.bounds.is_empty() {
                        quote! { #param: ::salsa::DebugWithDb<_Db> }
                    } else {
                        quote! { #param + ::salsa::DebugWithDb<_Db> }
                    }),
                    syn::GenericParam::Const(_) => Some(quote! { #param }),
                    syn::GenericParam::Lifetime(_) | syn::GenericParam::Const(_) => None,
                }),
            );
        if lifetime_generics_punctuated.is_empty() {
            quote! { _Db:  #db_path + ?Sized, #nonlifetime_generics_punctuated }
        } else {
            quote! { #lifetime_generics_punctuated, _Db:  #db_path + ?Sized, #nonlifetime_generics_punctuated }
        }
    }
}
