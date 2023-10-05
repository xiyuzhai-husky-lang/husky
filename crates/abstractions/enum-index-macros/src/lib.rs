// #[proc_macro_attribute]

#[proc_macro_derive(IsEnumIndex)]
pub fn derive_is_enum_index(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let syn::Item::Enum(ref item) = syn::parse_macro_input!(input as syn::Item) else {
        panic!("expect enum")
    };
    let generics = &item.generics;
    let ty_ident = &item.ident;
    let n = &item.variants.len();
    for variant in &item.variants {
        if variant.discriminant.is_some() {
            panic!("expect no discriminant")
        }
        match variant.fields {
            syn::Fields::Unit => (),
            syn::Fields::Named(_) | syn::Fields::Unnamed(_) => {
                panic!("expect all variants to be unit")
            }
        }
    }
    quote::quote! {
        impl #generics enum_index::IsEnumIndex for #ty_ident #generics {
            const N: usize = #n;

            fn from_index(index: usize) -> Self {
                debug_assert!(index < Self::N);
                unsafe { std::mem::transmute(index as u8) }
            }

            fn index(self) -> usize {
                (unsafe { std::mem::transmute::<_, u8>(self) }) as usize
            }
        }
    }
    .into()
}
