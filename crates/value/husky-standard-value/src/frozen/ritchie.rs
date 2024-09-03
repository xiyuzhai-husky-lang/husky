use super::*;

macro_rules! impl_frozen_for_ritchie_ty {
    (
        [$($input:ident),*], $output:ident
    ) => {
        impl<$($input,)* $output>  Frozen for fn($($input,)*) -> $output
        where
            $($input: Thawed, )*
            $output: Thawed, {
            type Thawed = Self;
            type Slush = ();

            fn thaw(&self) -> (Option<Self::Slush>, Self::Thawed) {
                (None, *self)
            }
        }
    };
}

for_all_ritchie_tys!(impl_frozen_for_ritchie_ty);
