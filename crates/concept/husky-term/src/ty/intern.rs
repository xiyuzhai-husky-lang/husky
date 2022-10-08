use crate::Ty;
use interner::{DefaultInternedPtr, Interner};

pub type TyInterner = Interner<TyPtr>;

pub type TyPtr = DefaultInternedPtr<Ty, Ty>;

pub trait InternTy {
    fn interner(&self) -> &TyInterner;
    fn intern_ty(&self, ty: Ty) -> TyPtr {
        todo!()
    }
}
