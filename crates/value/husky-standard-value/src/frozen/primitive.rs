use super::*;

macro_rules! impl_frozen_for_primitive_ty {
    ($primitive_ty: ty) => {
        impl Frozen for $primitive_ty {
            type Thawed = Self;
            type Slush = ();

            fn thaw(&self) -> (Option<Self::Slush>, Self::Thawed) {
                (None, *self)
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

for_all_primitive_tys!(impl_frozen_for_primitive_ty);
