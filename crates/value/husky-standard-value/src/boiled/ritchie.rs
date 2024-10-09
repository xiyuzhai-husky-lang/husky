use super::*;

macro_rules! impl_boiled_for_ritchie_ty {
    (
        [$($input:ident),*], $output:ident
    ) => {
        impl<$($input,)* $output> Boiled for fn($($input,)*) -> $output
        where
            $($input: Boiled, )*
            $output: Boiled, {
            type Thawed = fn($(<$input as Boiled>::Thawed,)*) -> <$output as Boiled>::Thawed;
unsafe fn from_thawed(thawed: Self::Thawed) -> Self {
                std::mem::transmute(thawed)
            }

            #[inline]
            unsafe fn from_thawed_ref(thawed_ref: &Self::Thawed) -> &Self {
                std::mem::transmute(thawed_ref)
            }

            unsafe fn into_thawed(self) -> Self::Thawed {
                std::mem::transmute(self)
            }
        }
    };
}

for_all_ritchie_tys!(impl_boiled_for_ritchie_ty);
