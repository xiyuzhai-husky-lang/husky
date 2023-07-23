use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TypeAssociatedTypeHirDefn {
    #[id]
    pub path: TypeItemPath,
    pub decl: TypeAssociatedTypeHirDecl,
    pub expr_region: HirExprRegion,
}

impl TypeAssociatedTypeHirDefn {
    pub(super) fn new(
        db: &dyn HirDefnDb,
        path: TypeItemPath,
        decl: TypeAssociatedTypeHirDecl,
    ) -> HirDefnResult<Self> {
        todo!()
    }
}
