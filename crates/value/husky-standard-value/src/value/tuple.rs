use super::*;

macro_rules! impl_immortal_for_non_unit_tuple_ty {
    (
        $($field:ident),*
    ) => {
        impl<$($field,)*> Immortal for ($($field,)*)
        where
            $($field: Immortal,)*
        {
            fn try_copy(&self) -> Option<Value> {
                todo!()
            }
        }
    };
}

for_all_non_unit_tuple_tys!(impl_immortal_for_non_unit_tuple_ty);
