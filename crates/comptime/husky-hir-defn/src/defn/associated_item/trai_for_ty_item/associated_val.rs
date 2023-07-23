use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TraitForTypeAssociatedValHirDefn {
    #[id]
    pub path: TraitForTypeItemPath,
    pub decl: TraitForTypeAssociatedValHirDecl,
    pub expr_region: HirExprRegion,
}

impl TraitForTypeAssociatedValHirDefn {
    pub(super) fn new(
        db: &dyn HirDefnDb,
        path: TraitForTypeItemPath,
        decl: TraitForTypeAssociatedValHirDecl,
    ) -> HirDefnResult<Self> {
        todo!()
    }
}
