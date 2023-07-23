use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TraitAssociatedTypeHirDefn {
    #[id]
    pub path: TraitItemPath,
    pub decl: TraitAssociatedTypeHirDecl,
    pub expr_region: HirExprRegion,
}
