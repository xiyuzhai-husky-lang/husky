use super::*;

macro_rules! impl_boiled_for_primitive_ty {
    ($primitive_ty: ty) => {
        impl Boiled for $primitive_ty {
            type Thawed = Self;

            unsafe fn into_thawed(self) -> Self::Thawed {
                std::mem::transmute(self)
            }
        }
    };
}

for_all_primitive_tys!(impl_boiled_for_primitive_ty);
