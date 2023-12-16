use husky_macro_utils::self_ty;
use proc_macro2::Span;
use quote::quote;

pub(crate) fn value(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = syn::parse_macro_input!(input as syn::Item);
    let (ident, generics) = match item {
        syn::Item::Enum(ref item) => (&item.ident, &item.generics),
        syn::Item::Struct(_) => todo!(),
        syn::Item::Type(_) => todo!(),
        _ => panic!(),
    };
    let generics_params = &generics.params;
    let generics_with_t = if generics_params.is_empty() {
        quote! {
            <__T>
        }
    } else {
        quote! {
            <#generics_params, __T>
        }
    };
    let generics_with_temp_lifetime_and_t = if generics_params.is_empty() {
        quote! {
            <'__temp, __T>
        }
    } else {
        quote! {
            <'__temp, #generics_params, __T>
        }
    };
    let value_ty = self_ty(ident, generics);
    let primitive_ty_value_conversions = [
        "bool", "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128",
        "isize", "f32", "f64",
    ]
    .into_iter()
    .map(|ty_str| {
        let ty = syn::Ident::new(ty_str, Span::call_site());
        quote! {
            impl FromValue for #ty {
                fn from_value(value: #value_ty) -> Self {
                    value.into()
                }
            }

            impl IntoValue for #ty {
                fn into_value(self) -> #value_ty {
                    self.into()
                }
            }
        }
    })
    .collect::<proc_macro2::TokenStream>();
    quote! {
        #item

        pub trait FromValue #generics {
            fn from_value(value: #value_ty) -> Self;
        }

        pub trait IntoValue #generics {
            fn into_value(self) -> #value_ty;
        }


        #primitive_ty_value_conversions

        // repeat the above code with type u8 replaced by u8~u128,usize, i8~i128,isze


        impl #generics_with_temp_lifetime_and_t FromValue for &'__temp __T {
            fn from_value(value: #value_ty) -> Self {
                todo!()
            }
        }

        impl #generics_with_temp_lifetime_and_t IntoValue for &'__temp __T {
            fn into_value(self) -> #value_ty {
                todo!()
            }
        }

        impl #generics_with_temp_lifetime_and_t FromValue for &'__temp mut __T {
            fn from_value(value: #value_ty) -> Self {
                todo!()
            }
        }

        impl #generics_with_temp_lifetime_and_t IntoValue for &'__temp mut __T {
            fn into_value(self) -> #value_ty {
                todo!()
            }
        }

        impl #generics_with_t FromValue for Option<__T> {
            fn from_value(value: #value_ty) -> Self {
                todo!()
            }
        }

        impl #generics_with_t IntoValue for Option<__T> {
            fn into_value(self) -> #value_ty {
                todo!()
            }
        }

        impl #generics_with_t FromValue for Vec<__T> {
            fn from_value(value: #value_ty) -> Self {
                todo!()
            }
        }

        impl #generics_with_t IntoValue for Vec<__T> {
            fn into_value(self) -> #value_ty {
                todo!()
            }
        }

        // impl #generics_with_t From<CyclicSliceLeashed<__T>> for #self_ty {
        //     fn from(t: CyclicSliceLeashed<__T>) -> Self {
        //         todo!()
        //     }
        // }

        // impl #generics_with_t Into<CyclicSliceLeashed<__T>> for #self_ty {
        //     fn into(self) -> CyclicSliceLeashed<__T> {
        //         todo!()
        //     }
        // }
    }
    .into()
}
