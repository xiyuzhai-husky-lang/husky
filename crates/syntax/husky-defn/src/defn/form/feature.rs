use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct FeatureDefn {
    #[id]
    pub path: FormPath,
    pub decl: FeatureDecl,
    pub expr_region: ExprRegion,
    pub body: DefnResult<ExprIdx>,
}
