use super::*;

macro_rules! impl_thawed_for_non_unit_tuple_ty {
    (
        $($field:ident),*
    ) => {
        impl<$($field,)*> Thawed for ($($field,)*)
        where
            $($field: Thawed,)*
        {
            type Frozen = ($(<$field as Thawed>::Frozen,)*);

            fn is_copyable() -> bool {
                todo!()
            }

            fn try_copy_thawed(&self) -> Option<ThawedValue> {
                todo!()
            }

             fn freeze(&self) -> Self::Frozen {
                todo!()
            }

            fn serialize_to_value(&self) -> serde_json::Value {
                todo!("impl_thawed_for_non_unit_tuple_ty serialize_to_value")
            }

            fn visualize_or_void(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
                todo!()
            }
        }
    };
}

for_all_non_unit_tuple_tys!(impl_thawed_for_non_unit_tuple_ty);
