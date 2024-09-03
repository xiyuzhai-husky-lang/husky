use super::*;

macro_rules! impl_immortal_for_non_unit_tuple_ty {
    (
        $($field:ident),*
    ) => {
        impl<$($field,)*> Immortal for ($($field,)*)
        where
            $($field: Immortal,)*
        {
            fn is_copyable() -> bool {
                todo!()
            }

            fn try_copy(&self) -> Option<Value> {
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

for_all_non_unit_tuple_tys!(impl_immortal_for_non_unit_tuple_ty);
