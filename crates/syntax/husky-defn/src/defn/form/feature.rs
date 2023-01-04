use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct FeatureDefn {
    #[id]
    pub path: FormPath,
    pub decl: FeatureDecl,
    pub expr_sheet: ExprSheet,
    pub body: DefnResult<ExprIdx>,
}
