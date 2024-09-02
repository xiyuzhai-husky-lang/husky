use super::*;

macro_rules! impl_frozen_for_primitive_ty {
    ($primitive_ty: ty) => {
        impl Frozen for $primitive_ty {
            type Thawed = Self;
            type Slush = ();

            fn thaw(&self) -> (Option<Self::Slush>, Self::Thawed) {
                (None, *self)
            }
        }
    };
}

for_all_primitive_tys!(impl_frozen_for_primitive_ty);
