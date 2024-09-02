use super::*;

macro_rules! impl_thawed_for_primitive_ty {
    ($primitive_ty: ty) => {
        impl Thawed for $primitive_ty {
            type Frozen = Self;

            fn is_copyable() -> bool {
                true
            }

            fn try_copy(&self) -> Option<Value> {
                Some((*self).into())
            }

            unsafe fn freeze(&self) -> Self::Frozen {
                *self
            }

            fn serialize_to_value(&self) -> serde_json::Value {
                serde_json::to_value(self).unwrap()
            }

            fn visualize_or_void(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
                todo!("")
            }
        }
    };
}

for_all_primitive_tys!(impl_thawed_for_primitive_ty);
