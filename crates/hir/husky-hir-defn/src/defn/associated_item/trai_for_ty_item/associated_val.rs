use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TraitForTypeAssociatedValHirDefn {
    pub path: TraitForTypeItemPath,
    pub hir_decl: TraitForTypeAssociatedValHirDecl,
    pub hir_expr_region: HirEagerExprRegion,
}

impl TraitForTypeAssociatedValHirDefn {
    pub(super) fn new(
        _db: &dyn HirDefnDb,
        _path: TraitForTypeItemPath,
        _hir_decl: TraitForTypeAssociatedValHirDecl,
    ) -> Self {
        todo!()
    }
}
