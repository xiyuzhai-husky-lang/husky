use super::*;

macro_rules! impl_frozen_for_ritchie_ty {
    (
        [$($input:ident),*], $output:ident
    ) => {
        impl<$($input,)* $output>  Frozen for fn($($input,)*) -> $output
        where
            $($input: 'static, )*
            $output: 'static, {
            type Thawed = Self;
            type Slush = ();

            fn thaw(&self) -> (Option<Self::Slush>, Self::Thawed) {
                (None, *self)
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

for_all_ritchie_tys!(impl_frozen_for_ritchie_ty);
