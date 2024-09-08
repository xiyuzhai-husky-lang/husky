use super::*;

macro_rules! impl_immortal_for_ritchie_ty {
    (
        [$($input:ident),*], $output:ident
    ) => {
        impl<$($input,)* $output> Immortal for fn($($input,)*) -> $output
        where
            $($input: 'static, )*
            $output: 'static, {
            fn try_copy(&self) -> Option<Value> {
                todo!()
            }
        }
    };
}

for_all_ritchie_tys!(impl_immortal_for_ritchie_ty);
