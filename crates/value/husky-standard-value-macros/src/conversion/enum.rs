use super::*;

pub(super) fn enum_value_conversion(item: syn::ItemEnum) -> TokenStream {
    let syn::ItemEnum {
        attrs: _,
        vis: _,
        enum_token: _,
        ref ident,
        ref generics,
        brace_token: _,
        ref variants,
    } = item;
    let self_ty = self_ty(ident, generics);
    let impl_immortal_generic_constraints = impl_immortal_generic_constraints(generics);
    let impl_boiled_generic_constraints = impl_boiled_generic_constraints(generics);
    let impl_boiled_assoc_ty_static = impl_boiled_assoc_thawed(ident, generics);
    let impl_thawed_generic_constraints = impl_thawed_generic_constraints(generics);
    let impl_thawed_assoc_ty_frozen = impl_thawed_assoc_frozen(ident, generics);
    let impl_frozen_generic_constraints = impl_frozen_generic_constraints(generics);
    let impl_frozen_assoc_ty_static = impl_frozen_assoc_thawed(ident, generics);
    let impl_from_value_generic_constraints = impl_from_value_generic_constraints(generics);
    let impl_into_value_generic_constraints = impl_into_value_generic_constraints(generics);
    let impl_from_thawed_value_generic_constraints =
        impl_from_thawed_value_generic_constraints(generics);
    let impl_into_thawed_value_generic_constraints =
        impl_into_thawed_value_generic_constraints(generics);
    let is_trivial = variants.iter().all(|variant| match variant.fields {
        syn::Fields::Unit => true,
        syn::Fields::Named(_) | syn::Fields::Unnamed(_) => false,
    });
    if is_trivial {
        quote::quote! {
            #[derive(__Serialize)]
            #[serde(crate = "self::serde")]
            #item

            impl #generics __Immortal for #self_ty where #impl_immortal_generic_constraints {
                fn try_copy(&self) -> Option<__Value> {
                    todo!()
                }
            }

            impl #generics __Boiled for #self_ty where #impl_boiled_generic_constraints {
                type Thawed = #impl_boiled_assoc_ty_static;

                unsafe fn from_thawed(thawed: Self::Thawed) -> Self {
                    std::mem::transmute(thawed)
                }

                unsafe fn into_thawed(self) -> Self::Thawed {
                    std::mem::transmute(self)
                }
            }

            impl #generics __Thawed for #self_ty where #impl_thawed_generic_constraints {
                type Frozen = #impl_thawed_assoc_ty_frozen;

                fn is_copyable() -> bool {
                    todo!()
                }

                fn try_copy_thawed(&self) -> Option<__ThawedValue> {
                    todo!()
                }

                 fn freeze(&self) -> Self::Frozen {
                    // FrozenMut::new(*self)
                    todo!()
                }
            }

            impl #generics __Frozen for #self_ty where #impl_frozen_generic_constraints {
                type Thawed = #impl_frozen_assoc_ty_static;

                type Slush = ();

                fn thaw(&self) -> (Option<Self::Slush>, Self::Thawed) {
                    todo!()
                }

                fn serialize_to_value(&self) -> __JsonValue {
                    __to_json_value(self).unwrap()
                }

                fn visualize_or_void(&self, visual_synchrotron: &mut __VisualSynchrotron) -> __Visual {
                    // ad hoc
                    __Visual::Void
                }
            }

            impl #generics __FromValue for #self_ty {
                fn from_value_aux(value: __Value, _slush_values: Option<&mut __SlushValues>) -> Self {
                    let __Value::EnumUnit { index, .. } = value else {
                        unreachable!()
                    };
                    let index: u8 = index.try_into().unwrap();
                    unsafe {
                        std::mem::transmute(index)
                    }
                }
            }

            impl #generics __IntoValue for #self_ty {
                fn into_value(self) -> __Value {
                    let index: u8 = unsafe { std::mem::transmute(self) };
                    __Value::from_enum_index(index as usize, |index: usize, _, _| {
                        let index: u8 = index.try_into().unwrap();
                        let slf: Self = unsafe { std::mem::transmute(index) };
                        __ValuePresentation::AdHoc(format!("{slf:?}"))
                    })
                }
            }

            impl #generics __FromThawedValue for #self_ty where #impl_from_thawed_value_generic_constraints  {
                fn from_thawed_value_aux(value: __ThawedValue, _slush_values: Option<&mut __SlushValues>) -> Self {
                    let __ThawedValue::EnumUnit { index, .. } = value else {
                        unreachable!()
                    };
                    let index: u8 = index.try_into().unwrap();
                    unsafe {
                        std::mem::transmute(index)
                    }
                }
            }

            impl #generics __IntoThawedValue for #self_ty where #impl_into_thawed_value_generic_constraints  {
                fn into_thawed_value(self) -> __ThawedValue {
                    let index: u8 = unsafe { std::mem::transmute(self) };
                    __ThawedValue::from_enum_index(index as usize, |index: usize, _, _| {
                        let index: u8 = index.try_into().unwrap();
                        let slf: Self = unsafe { std::mem::transmute(index) };
                        __ValuePresentation::AdHoc(format!("{slf:?}"))
                    })
                }
            }
        }
        .into()
    } else {
        quote::quote! {
            #[derive(__Serialize)]
            #[serde(crate = "self::serde")]
            #item

            impl #generics __Immortal for #self_ty where #impl_boiled_generic_constraints {
                fn try_copy(&self) -> Option<__Value> {
                    todo!()
                }
            }

            impl #generics __Boiled for #self_ty where #impl_boiled_generic_constraints {
                type Thawed = #impl_boiled_assoc_ty_static;

                unsafe fn from_thawed(thawed: Self::Thawed) -> Self {
                    std::mem::transmute(thawed)
                }

                unsafe fn into_thawed(self) -> Self::Thawed {
                    std::mem::transmute(self)
                }
            }

            impl #generics __Thawed for #self_ty where #impl_thawed_generic_constraints {
                type Frozen = #impl_thawed_assoc_ty_frozen;

                fn is_copyable() -> bool {
                    todo!()
                }

                fn try_copy_thawed(&self) -> Option<__ThawedValue> {
                    todo!()
                }

                 fn freeze(&self) -> Self::Frozen {
                    // FrozenMut::new(*self)
                    todo!()
                }
            }

            impl #generics __Frozen for #self_ty where #impl_frozen_generic_constraints {
                type Thawed = #impl_frozen_assoc_ty_static;

                type Slush = ();

                fn thaw(&self) -> (Option<Self::Slush>, Self::Thawed) {
                    todo!()
                }

                fn serialize_to_value(&self) -> __JsonValue {
                    __to_json_value(self).unwrap()
                }

                fn visualize_or_void(&self, visual_synchrotron: &mut __VisualSynchrotron) -> __Visual {
                    // ad hoc
                    __Visual::Void
                }
            }

            impl #generics __FromValue for #self_ty where #impl_from_value_generic_constraints {
                fn from_value_aux(value: __Value, _: Option<&mut __SlushValues>) -> Self {
                    value.into_owned()
                }
            }

            impl #generics __IntoValue for #self_ty where #impl_into_value_generic_constraints {
                fn into_value(self) -> __Value {
                    __Value::from_owned(self)
                }
            }

            impl #generics __FromThawedValue for #self_ty where #impl_from_thawed_value_generic_constraints {
                fn from_thawed_value_aux(value: __ThawedValue, _: Option<&mut __SlushValues>) -> Self {
                    value.into_owned()
                }
            }

            impl #generics __IntoThawedValue for #self_ty where #impl_into_thawed_value_generic_constraints {
                fn into_thawed_value(self) -> __ThawedValue {
                    __ThawedValue::from_owned(self)
                }
            }
        }
        .into()
    }
}
