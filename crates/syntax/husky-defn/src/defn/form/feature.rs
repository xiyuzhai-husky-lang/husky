use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct FeatureDefn {
    #[id]
    pub path: FormPath,
    pub decl: FeatureDecl,
    pub variable_sheet: VariableSheet,
    pub expr_sheet: ExprSheet,
    pub body: DefnResult<ExprIdx>,
}
