use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct FeatureDefn {
    #[id]
    pub path: FormPath,
    pub decl: FeatureDecl,
    pub expr_page: ExprPage,
    pub body: DefnResult<ExprIdx>,
}
