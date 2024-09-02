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
        }
    };
}

for_all_non_unit_tuple_tys!(impl_frozen_for_tuple_ty);
