use husky_macro_utils::generics_without_bounds;
use proc_macro::TokenStream;

pub fn value_conversion(args: TokenStream, input: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(input as syn::Item);
    match item {
        syn::Item::Enum(item) => enum_value_conversion(item),
        syn::Item::Struct(item) => struct_value_conversion(item),
        _ => todo!(),
    }
}

fn struct_value_conversion(item: syn::ItemStruct) -> TokenStream {
    let syn::ItemStruct {
        ref attrs,
        ref vis,
        struct_token,
        ref ident,
        ref generics,
        ref fields,
        semi_token,
    } = item;
    let generics_without_bounds = generics_without_bounds(generics);
    quote::quote! {
        #item

        // todo: value generics
        impl #generics __FromValue for #ident #generics_without_bounds {
            fn from_value(value: __Value) -> Self {
                // Value::from_owned(self)
                todo!()
            }
        }

        // todo: value generics
        impl #generics __IntoValue for #ident #generics_without_bounds {
            fn into_value(self) -> __Value {
                // Value::from_owned(self)
                todo!()
            }
        }
    }
    .into()
}

fn enum_value_conversion(item: syn::ItemEnum) -> TokenStream {
    let syn::ItemEnum {
        ref attrs,
        ref vis,
        enum_token,
        ref ident,
        ref generics,
        brace_token,
        ref variants,
    } = item;
    let generics_without_bounds = generics_without_bounds(generics);
    let is_trivial = variants.iter().all(|variant| match variant.fields {
        syn::Fields::Unit => true,
        syn::Fields::Named(_) | syn::Fields::Unnamed(_) => false,
    });
    if is_trivial {
        quote::quote! {
            #item

            impl #generics __FromValue for #ident #generics_without_bounds {
                fn from_value(value: __Value) -> Self {
                    let __Value::EnumU8(index_raw) = value else {
                        unreachable!()
                    };
                    unsafe {
                        std::mem::transmute(index_raw)
                    }
                }
            }

            impl #generics __IntoValue for #ident #generics_without_bounds {
                fn into_value(self) -> __Value {
                    __Value::EnumU8(unsafe {
                        std::mem::transmute(self)
                    })
                }
            }
        }
        .into()
    } else {
        quote::quote! {
            #item

            // todo: value generics
            impl #generics __FromValue for #ident #generics_without_bounds {
                fn from_value(value: __Value) -> Self {
                    // ad hoc
                    // let __Value::EnumU8(index_raw) = value else {
                    //     unreachable!()
                    // };
                    // unsafe {
                    //     std::mem::transmute(index_raw)
                    // }
                    todo!()
                }
            }

            // todo: value generics
            impl #generics __IntoValue for #ident #generics_without_bounds {
                fn into_value(self) -> __Value {
                    // ad hoc
                    todo!("enum into value")
                }
            }
        }
        .into()
    }
}
