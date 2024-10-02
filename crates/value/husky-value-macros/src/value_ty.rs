use husky_proc_macro_utils::self_ty;
use quote::quote;

pub(crate) fn value_ty(
    _args: proc_macro::TokenStream,
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
    let value_trai = syn::Ident::new("Immortal", ident.span());
    let from_value_trai = syn::Ident::new("FromValue", ident.span());
    let into_value_trai = syn::Ident::new("IntoValue", ident.span());
    let primitive_ty_value_conversions = [
        "()", "bool", "char", "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64",
        "i128", "isize", "f32", "f64",
    ]
    .into_iter()
    .map(|ty_str| {
        let ty: proc_macro2::TokenStream = ty_str.parse().unwrap();
        quote! {
            impl #from_value_trai for #ty {
                fn from_value_aux(value: #value_ty, _slush_values: Option<&mut SlushValues>) -> Self {
                    value.into()
                }
            }

            impl #into_value_trai for #ty {
                fn into_value(self) -> #value_ty {
                    self.into()
                }
            }
        }
    })
    .collect::<proc_macro2::TokenStream>();
    quote! {
        #item

        pub trait #from_value_trai: Sized {
            /// `slush_values` is needed for keeping memory valid when coersing owned ty into ref or ref mut
            fn from_value_aux(value: #value_ty, slush_values: Option<&mut SlushValues>) -> Self;

            /// final
            fn from_value_static(value: #value_ty) -> Self {
                Self::from_value_aux(value, None)
            }

            /// final
            fn from_value_temp(value: #value_ty, slush_values: &mut SlushValues) -> Self {
                Self::from_value_aux(value, Some(slush_values))
            }

            /// this is useful for keyed argument,
            /// only implement this for Option
            fn from_optional_value(value: Option<#value_ty>) -> Self {
                panic!("can't be constructed from optional value")
            }

            /// this is useful for variadic argument,
            // only implement this for Vec, SmallVec
            fn from_variadic_values<E>(
                values: impl Iterator<Item = KiControlFlow<#value_ty, #value_ty, E>>,
                slush_values: Option<&mut SlushValues>,
            ) -> KiControlFlow<Self, #value_ty, E> {
                panic!("can't be constructed from value iter")
            }
        }

        impl #from_value_trai for #value_ty {
            fn from_value_aux(value: #value_ty, _slush_values: Option<&mut SlushValues>) -> Self {
                value
            }
        }

        pub trait #into_value_trai: Sized {
            fn into_value(self) -> #value_ty;
        }

        #primitive_ty_value_conversions

        impl #from_value_trai for &'static str {
            fn from_value_aux(value: #value_ty, slush_values: Option<&mut SlushValues>) -> Self {
                todo!()
            }
        }

        impl #into_value_trai for &'static str {
            fn into_value(self) -> #value_ty {
                todo!()
            }
        }

        impl #generics_with_t #from_value_trai for Option<__T> where __T: #value_trai {
            fn from_value_aux(value: #value_ty, _slush_values: Option<&mut SlushValues>) -> Self {
                println!("__T typename = {}", std::any::type_name::<__T>());
                todo!("impl #generics_with_t #from_value_trai for Option<__T>")
            }

            /// this is useful for keyed argument,
            /// only implement this for Option
            fn from_optional_value(value: Option<#value_ty>) -> Self {
                todo!("Option from_optional_value")
            }
        }

        impl #generics_with_t #into_value_trai for Option<__T> where __T: #value_trai {
            fn into_value(self) -> #value_ty {
                println!("__T typename = {}", std::any::type_name::<__T>());
                todo!("impl #generics_with_t #into_value_trai for Option<__T>")
            }
        }

        impl #generics_with_t #from_value_trai for Vec<__T> where __T: #value_trai {
            fn from_value_aux(value: #value_ty, _slush_values: Option<&mut SlushValues>) -> Self {
                println!("__T typename = {}", std::any::type_name::<__T>());
                todo!("impl #generics_with_t #from_value_trai for Vec<__T>")
            }

            /// this is useful for variadic argument,
            // only implement this for Vec, SmallVec
            fn from_variadic_values<E>(
                values: impl Iterator<Item = KiControlFlow<#value_ty, #value_ty, E>>,
                slush_values: Option<&mut SlushValues>,
            ) -> KiControlFlow<Self, #value_ty, E> {
                match slush_values {
                    Some(slush_values) => values.map(
                        |ki_control_flow| ki_control_flow.map(
                            |v|__T::from_value_temp(v, slush_values)
                        )
                    ).collect(),
                    None => values.map(
                        |ki_control_flow| ki_control_flow.map(
                            |v|__T::from_value_static(v)
                        )
                    ).collect(),
                }
            }
        }

        impl #generics_with_t #into_value_trai for Vec<__T> where __T: #value_trai {
            fn into_value(self) -> #value_ty {
                #value_ty::from_owned(self)
            }
        }

        impl<C, B> #into_value_trai for std::ops::ControlFlow<B, C>
        where C: #value_trai,
            B: #value_trai
        {
            fn into_value(self) -> #value_ty {
                todo!("impl<C, B> #into_value_trai for std::ops::ControlFlow<B, C>")
            }
        }

        macro_rules! impl_ritchie_fn_value_conversion {
            ([$($input: ident),*], $output: ident) => {
                impl<$($input,)* $output> #from_value_trai for fn($($input,)*) -> $output {
                    fn from_value_aux(value: #value_ty, _slush_values: Option<&mut SlushValues>) -> Self {
                        todo!("impl_ritchie_fn_value_conversion #from_value_trai")
                    }
                }

                impl<$($input,)* $output> #into_value_trai for fn($($input,)*) -> $output {
                    fn into_value(self) -> #value_ty {
                        todo!("impl_ritchie_fn_value_conversion #into_value_trai")
                    }
                }
            };
        }

        for_all_ritchie_tys! { impl_ritchie_fn_value_conversion }

        macro_rules! impl_non_unit_tuple_value_conversion {
            ($($field: ident),*) => {
                impl<$($field,)*> #from_value_trai for ($($field,)*) {
                    fn from_value_aux(value: #value_ty, _slush_values: Option<&mut SlushValues>) -> Self {
                        todo!("impl_ritchie_fn_value_conversion #from_value_trai")
                    }
                }

                impl<$($field,)*> #into_value_trai for ($($field,)*) {
                    fn into_value(self) -> #value_ty {
                        todo!("impl_ritchie_fn_value_conversion #into_value_trai")
                    }
                }
            };
        }

        for_all_non_unit_tuple_tys! { impl_non_unit_tuple_value_conversion }

        impl #generics_with_temp_lifetime_and_t #from_value_trai for &'__temp __T where __T: #value_trai {
            fn from_value_aux(value: #value_ty, slush_values: Option<&mut SlushValues>) -> Self {
                value.into_ref(slush_values)
            }
        }

        impl #generics_with_temp_lifetime_and_t #from_value_trai for &'__temp mut __T where __T: #value_trai {
            fn from_value_aux(value: #value_ty, _slush_values: Option<&mut SlushValues>) -> Self {
                println!("__T typename = {}", std::any::type_name::<__T>());
                todo!("impl #generics_with_t #from_value_trai for &'temp mut __T")
            }
        }

        impl #generics_with_temp_lifetime_and_t #from_value_trai for &'__temp [__T] where __T: #value_trai {
            fn from_value_aux(value: #value_ty, _slush_values: Option<&mut SlushValues>) -> Self {
                println!("__T typename = {}", std::any::type_name::<__T>());
                todo!("impl #generics_with_temp_lifetime_and_t #from_thawed_value_trai for &'__temp [__T]")
            }
        }

        // TODO ad hoc
        impl #generics_with_temp_lifetime_and_t #from_value_trai for &'__temp mut [__T] where __T: #value_trai {
            fn from_value_aux(value: #value_ty, _slush_values: Option<&mut SlushValues>) -> Self {
                println!("__T typename = {}", std::any::type_name::<__T>());
                todo!("impl #generics_with_temp_lifetime_and_t #from_thawed_value_trai for &'__temp mut [__T]")
            }
        }
    }
    .into()
}
