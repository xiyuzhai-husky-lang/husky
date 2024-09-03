use super::*;

macro_rules! impl_immortal_for_ritchie_ty {
    (
        [$($input:ident),*], $output:ident
    ) => {
        impl<$($input,)* $output> Immortal for fn($($input,)*) -> $output
        where
            $($input: 'static, )*
            $output: 'static, {
            fn is_copyable() -> bool {
                true
            }

            fn try_copy(&self) -> Option<Value> {
                todo!()
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

for_all_ritchie_tys!(impl_immortal_for_ritchie_ty);

#[test]
fn it_works() {
    assert!(<fn() as Immortal>::is_copyable());
    assert!(<fn(&'static [i32]) as Immortal>::is_copyable());
    assert!(<fn(&'static [f32]) -> f32 as Immortal>::is_copyable());
    assert!(<fn(&'static [Vec<f32>]) -> &'static Vec<f32> as Immortal>::is_copyable());
}
