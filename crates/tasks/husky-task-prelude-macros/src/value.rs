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
        "()", "bool", "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64",
        "i128", "isize", "f32", "f64",
    ]
    .into_iter()
    .map(|ty_str| {
        let ty: proc_macro2::TokenStream = ty_str.parse().unwrap();
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


        impl #generics_with_temp_lifetime_and_t FromValue for &'__temp __T where __T: WeakStatic {
            fn from_value(value: #value_ty) -> Self {
                value.into_ref()
            }
        }

        impl #generics_with_temp_lifetime_and_t IntoValue for &'__temp __T where __T: WeakStatic {
            fn into_value(self) -> #value_ty {
                println!("__T typename = {}", std::any::type_name::<__T>());
                todo!("impl #generics_with_temp_lifetime_and_t IntoValue for &'__temp __T")
            }
        }

        impl #generics_with_temp_lifetime_and_t FromValue for &'__temp mut __T where __T: WeakStatic {
            fn from_value(value: #value_ty) -> Self {
                println!("__T typename = {}", std::any::type_name::<__T>());
                todo!("impl #generics_with_temp_lifetime_and_t FromValue for &'__temp mut __T")
            }
        }

        impl #generics_with_temp_lifetime_and_t IntoValue for &'__temp mut __T where __T: WeakStatic {
            fn into_value(self) -> #value_ty {
                println!("__T typename = {}", std::any::type_name::<__T>());
                todo!("impl #generics_with_temp_lifetime_and_t IntoValue for &'__temp mut __T")
            }
        }

        impl #generics_with_t FromValue for Option<__T> where __T: WeakStatic {
            fn from_value(value: #value_ty) -> Self {
                println!("__T typename = {}", std::any::type_name::<__T>());
                todo!("impl #generics_with_t FromValue for Option<__T>")
            }
        }

        impl #generics_with_t IntoValue for Option<__T> where __T: WeakStatic {
            fn into_value(self) -> #value_ty {
                println!("__T typename = {}", std::any::type_name::<__T>());
                todo!("impl #generics_with_t IntoValue for Option<__T>")
            }
        }

        impl #generics_with_t FromValue for Vec<__T> where __T: WeakStatic {
            fn from_value(value: #value_ty) -> Self {
                println!("__T typename = {}", std::any::type_name::<__T>());
                todo!("impl #generics_with_t FromValue for Vec<__T>")
            }
        }

        impl #generics_with_t IntoValue for Vec<__T> where __T: Static {
            fn into_value(self) -> #value_ty {
                #value_ty::from_owned(self)
            }
        }

        impl #generics_with_temp_lifetime_and_t FromValue for &'__temp [__T] where __T: WeakStatic {
            fn from_value(value: #value_ty) -> Self {
                println!("__T typename = {}", std::any::type_name::<__T>());
                todo!("impl #generics_with_temp_lifetime_and_t FromValue for &'__temp [__T]")
            }
        }

        impl #generics_with_temp_lifetime_and_t IntoValue for &'__temp [__T] where __T: WeakStatic {
            fn into_value(self) -> #value_ty {
                println!("__T typename = {}", std::any::type_name::<__T>());
                todo!("impl #generics_with_temp_lifetime_and_t IntoValue for &'__temp [__T]")
            }
        }

        impl #generics_with_temp_lifetime_and_t FromValue for &'__temp mut [__T] where __T: WeakStatic {
            fn from_value(value: #value_ty) -> Self {
                println!("__T typename = {}", std::any::type_name::<__T>());
                todo!("impl #generics_with_temp_lifetime_and_t FromValue for &'__temp mut [__T]")
            }
        }

        impl #generics_with_temp_lifetime_and_t IntoValue for &'__temp mut [__T] where __T: WeakStatic {
            fn into_value(self) -> #value_ty {
                println!("__T typename = {}", std::any::type_name::<__T>());
                todo!("impl #generics_with_temp_lifetime_and_t IntoValue for &'__temp mut [__T]")
            }
        }

        impl<C, B> IntoValue for std::ops::ControlFlow<B, C> {
            fn into_value(self) -> #value_ty {
                todo!("impl<C, B> IntoValue for std::ops::ControlFlow<B, C>")
            }
        }

        macro_rules! impl_ritchie_fn_version_conversion {
            ([$($input: ident),*], $output: ident) => {
                impl<$($input,)* $output> FromValue for fn($($input,)*) -> $output {
                    fn from_value(value: #value_ty) -> Self {
                        todo!("impl_ritchie_fn_version_conversion FromValue")
                    }
                }

                impl<$($input,)* $output> IntoValue for fn($($input,)*) -> $output {
                    fn into_value(self) -> #value_ty {
                        todo!("impl_ritchie_fn_version_conversion IntoValue")
                    }
                }
            };
        }

        for_all_ritchie_tys! { impl_ritchie_fn_version_conversion }

        /// conversion into Value must go through this builder,
        /// so that we can distinguish `&'static T` from other types
        pub struct ValueLeashTest<T>(pub T);

        /// distinguish `&'static T` from other types
        impl<T> ValueLeashTest<&'static T> where T: Static {
            pub fn into_value(self)  -> #value_ty {
                #value_ty::from_leash(self.0)
            }
        }

        impl<T> IntoValue for ValueLeashTest<T> where T: IntoValue {
            /// fallback to use <T as IntoValue>::into_value
            fn into_value(self)  -> #value_ty {
                // can't use `self.0.into_value()`,
                // because rustc will interpret this as calling <&T as IntoValue>::into_value
                <T as IntoValue>::into_value(self.0)
            }
        }
    }
    .into()
}
