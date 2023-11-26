use syn::Path;

pub(super) fn generics_without_db(
    generics: &syn::Generics,
    db_trai: &Path,
) -> proc_macro2::TokenStream {
    if generics.params.is_empty() {
        quote! {}
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
                        quote! { #param: ::salsa::DebugWithDb }
                    } else {
                        quote! { #param + ::salsa::DebugWithDb }
                    }),
                    syn::GenericParam::Const(_) => Some(quote! { #param }),
                    syn::GenericParam::Lifetime(_) => None,
                }),
            );
        if lifetime_generics_punctuated.is_empty() {
            quote! { <#nonlifetime_generics_punctuated> }
        } else {
            quote! { <#lifetime_generics_punctuated, #nonlifetime_generics_punctuated> }
        }
    }
}
