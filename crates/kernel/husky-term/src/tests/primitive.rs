use std::sync::Arc;

use crate::*;

impl TermMenu {
    pub fn primitive_ty_decls(&self) -> impl Iterator<Item = (Ty, Arc<TyDecl>)> {
        use TyFamily::*;
        [
            (self.unit(), Arc::new(TyDecl::new(Physical))),
            (self.i32(), Arc::new(TyDecl::new(Physical))),
            (self.i64(), Arc::new(TyDecl::new(Physical))),
            (self.f32(), Arc::new(TyDecl::new(Physical))),
            (self.f64(), Arc::new(TyDecl::new(Physical))),
            (self.b32(), Arc::new(TyDecl::new(Physical))),
            (self.b64(), Arc::new(TyDecl::new(Physical))),
            (self.bool(), Arc::new(TyDecl::new(Physical))),
        ]
        .into_iter()
    }
}
