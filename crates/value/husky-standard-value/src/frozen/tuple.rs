use super::*;

macro_rules! impl_frozen_for_tuple_ty {
    (
        $($field:ident),*
    ) => {
        impl<$($field,)*> Frozen for ($($field,)*)
        where
            $($field: Frozen,)*
        {
            type Thawed = ($(<$field as Frozen>::Thawed,)*);
            type Slush = ($(<$field as Frozen>::Slush,)*);

            fn thaw(&self) -> (Option<Self::Slush>, Self::Thawed) {
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

for_all_non_unit_tuple_tys!(impl_frozen_for_tuple_ty);
