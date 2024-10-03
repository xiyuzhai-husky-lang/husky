use super::*;
use husky_decl_macro_utils::for_all_non_unit_tuple_tys;
use paste::paste;

macro_rules! impl_boiled_for_non_unit_tuple_ty {
    (
        $($field:ident),*
    ) => {
        paste! {
            impl<$($field,)*> Boiled for ($($field,)*)
            where
                $($field: Boiled,)*
            {
                type Thawed = ($(<$field as Boiled>::Thawed,)*);

                unsafe fn from_thawed(thawed: Self::Thawed) -> Self {
                    let ($([<$field:lower>],)*) = thawed;
                    (
                        $(<$field as Boiled>::from_thawed([<$field:lower>]),)*
                    )
                }

                #[inline]
                unsafe fn from_thawed_ref(thawed_ref: &Self::Thawed) -> &Self {
                    std::mem::transmute(thawed_ref)
                }

                unsafe fn into_thawed(self) -> Self::Thawed {
                    let ($([<$field:lower>],)*) = self;
                    (
                        $([<$field:lower>].into_thawed(),)*
                    )
                }
            }
        }
    };
}

for_all_non_unit_tuple_tys!(impl_boiled_for_non_unit_tuple_ty);
