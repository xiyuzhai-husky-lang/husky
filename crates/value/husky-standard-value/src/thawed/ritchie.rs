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

            unsafe fn freeze(&self) -> Self::Frozen {
                *self
            }

            fn serialize_to_value(&self) -> serde_json::Value {
                todo!("impl_thawed_for_ritchie_ty serialize_to_value")
            }

            fn visualize_or_void(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
                Visual::Void
            }
        }
    };
}

for_all_ritchie_tys!(impl_thawed_for_ritchie_ty);
