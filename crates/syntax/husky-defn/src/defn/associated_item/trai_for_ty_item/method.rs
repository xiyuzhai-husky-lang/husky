use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TypeAsTraitMethodDefn {
    #[id]
    pub path: Option<TraitForTypeItemPath>,
    pub decl: TypeAsTraitMethodDecl,
    pub expr_region: ExprRegion,
    pub body: DefnResult<ExprIdx>,
}
