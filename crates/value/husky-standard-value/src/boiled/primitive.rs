use super::*;

macro_rules! impl_boiled_for_primitive_ty {
    ($primitive_ty: ty) => {
        impl Boiled for $primitive_ty {
            type Thawed = Self;
            unsafe fn from_thawed(thawed: Self::Thawed) -> Self {
                std::mem::transmute(thawed)
            }

            #[inline]
            unsafe fn from_thawed_ref(thawed_ref: &Self::Thawed) -> &Self {
                std::mem::transmute(thawed_ref)
            }

            unsafe fn into_thawed(self) -> Self::Thawed {
                self
            }
        }
    };
}

for_all_primitive_tys!(impl_boiled_for_primitive_ty);
