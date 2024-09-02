use super::*;

pub(super) fn struct_value_conversion(item: syn::ItemStruct) -> TokenStream {
    let syn::ItemStruct {
        attrs: _,
        vis: _,
        struct_token: _,
        ref ident,
        ref generics,
        fields: _,
        semi_token: _,
    } = item;
    let self_ty = self_ty(ident, generics);
    let impl_boiled_generic_constraints = impl_boiled_generic_constraints(generics);
    let impl_boiled_assoc_ty_static = impl_boiled_assoc_ty_static(ident, generics);
    let impl_thawed_generic_constraints = impl_thawed_generic_constraints(generics);
    let impl_thawed_assoc_ty_frozen = impl_thawed_assoc_ty_frozen(ident, generics);
    let impl_frozen_generic_constraints = impl_frozen_generic_constraints(generics);
    let impl_frozen_assoc_ty_static = impl_frozen_assoc_ty_static(ident, generics);
    let impl_from_value_generic_constraints = impl_from_value_generic_constraints(generics);
    let impl_into_value_generic_constraints = impl_into_value_generic_constraints(generics);
    quote::quote! {
        #[derive(__Serialize)]
        #[serde(crate = "self::serde")]
        #item

        impl #generics __Boiled for #self_ty where #impl_boiled_generic_constraints {
            type Thawed = #impl_boiled_assoc_ty_static;

            unsafe fn into_thawed(self) -> Self::Thawed {
                self
            }
        }

        impl #generics __Thawed for #self_ty where #impl_thawed_generic_constraints {
            type Frozen = #impl_thawed_assoc_ty_frozen;

            unsafe fn freeze(&self) -> Self::Frozen {
                // FrozenMut::new(*self)
                todo!()
            }

            fn is_copyable() -> bool {
                todo!()
            }

            fn try_copy(&self) -> Option<__Value> {
                todo!()
            }


            fn serialize_to_value(&self) -> __JsonValue {
                __to_json_value(self).unwrap()
            }

            fn visualize_or_void(&self, visual_synchrotron: &mut __VisualSynchrotron) -> __Visual {
                __VisualizeTest(self).visualize(visual_synchrotron)
            }
        }

        impl #generics __Frozen for #self_ty where #impl_frozen_generic_constraints {
            type Thawed = #impl_frozen_assoc_ty_static;

            type Slush = ();

            fn revive(&self) -> (Option<Self::Slush>, Self::Thawed) {
                todo!()
            }
        }

        // todo: value generics
        impl #generics __FromValue for #self_ty where #impl_from_value_generic_constraints {
            fn from_value_aux(value: __Value, _value_stands: Option<&mut __SlushValues>) -> Self {
                value.into_owned()
            }
        }

        // todo: value generics
        impl #generics __IntoValue for #self_ty where #impl_into_value_generic_constraints {
            fn into_value(self) -> __Value {
                __Value::from_owned(self)
            }
        }
    }
    .into()
}
