use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TypeAssociatedValHirDefn {
    #[id]
    pub path: TypeItemPath,
    pub decl: TypeAssociatedValHirDecl,
    pub expr_region: HirExprRegion,
}

impl TypeAssociatedValHirDefn {
    pub(super) fn new(
        db: &dyn HirDefnDb,
        path: TypeItemPath,
        decl: TypeAssociatedValHirDecl,
    ) -> HirDefnResult<Self> {
        todo!()
    }
}
