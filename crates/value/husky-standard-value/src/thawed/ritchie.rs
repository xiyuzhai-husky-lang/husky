use super::*;

macro_rules! impl_thawed_for_ritchie_ty {
    (
        [$($input:ident),*], $output:ident
    ) => {
        impl<$($input,)* $output> Thawed for fn($($input,)*) -> $output
        where
            $($input: 'static, )*
            $output: 'static, {
            type Frozen = Self;

            fn is_copyable() -> bool {
                todo!()
            }

            fn try_copy_thawed(&self) -> Option<ThawedValue> {
                todo!()
            }

             fn freeze(&self) -> Self::Frozen {
                *self
            }
        }
    };
}

for_all_ritchie_tys!(impl_thawed_for_ritchie_ty);
