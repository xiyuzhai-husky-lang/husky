use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct FeatureDefn {
    #[id]
    pub path: FormPath,
    pub decl: FeatureDecl,
    pub expr_region: ExprRegion,
    pub body: DefnResult<ExprIdx>,
}
