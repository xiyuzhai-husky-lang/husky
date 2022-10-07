use crate::*;

pub struct TyContext<'a> {
    db: &'a dyn TyQuery,
}

impl<'a> TyContext<'a> {
    fn ty_ty(&self, ty: TyPtr) -> TyPtr {
        self.db.intern_ty(ty.ty())
    }
}
