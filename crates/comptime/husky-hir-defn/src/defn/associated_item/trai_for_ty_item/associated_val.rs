use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TraitForTypeAssociatedValHirDefn {
    #[id]
    pub path: TraitForTypeItemPath,
    pub hir_decl: TraitForTypeAssociatedValHirDecl,
    pub hir_expr_region: HirEagerExprRegion,
}

impl TraitForTypeAssociatedValHirDefn {
    pub(super) fn new(
        db: &dyn HirDefnDb,
        path: TraitForTypeItemPath,
        hir_decl: TraitForTypeAssociatedValHirDecl,
    ) -> Self {
        todo!()
    }
}
