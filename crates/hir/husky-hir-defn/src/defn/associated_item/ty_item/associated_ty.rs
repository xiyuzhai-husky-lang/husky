use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TypeAssociatedTypeHirDefn {
    pub path: TypeItemPath,
    pub hir_decl: TypeAssociatedTypeHirDecl,
    pub hir_expr_region: HirEagerExprRegion,
}

impl TypeAssociatedTypeHirDefn {
    pub(super) fn new(
        _db: &dyn HirDefnDb,
        _path: TypeItemPath,
        _hir_decl: TypeAssociatedTypeHirDecl,
    ) -> Self {
        todo!()
    }
}
