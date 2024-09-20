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
        }
    };
}

for_all_non_unit_tuple_tys!(impl_thawed_for_non_unit_tuple_ty);
