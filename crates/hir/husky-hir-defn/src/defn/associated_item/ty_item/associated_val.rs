use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TypeAssociatedValHirDefn {
    pub path: TypeItemPath,
    pub hir_decl: TypeAssociatedValHirDecl,
    pub hir_expr_region: HirEagerExprRegion,
}

impl TypeAssociatedValHirDefn {
    pub(super) fn new(
        _db: &dyn HirDefnDb,
        _path: TypeItemPath,
        _hir_decl: TypeAssociatedValHirDecl,
    ) -> Self {
        todo!()
    }
}
