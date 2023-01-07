use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct FeatureDefn {
    #[id]
    pub path: FormPath,
    pub decl: FeatureDecl,
    pub expr_sheet: ModuleItemDefnExprSheet,
    pub body: DefnResult<ExprIdx>,
}

impl FeatureDefn {
    pub fn symbol_context(self, db: &dyn DeclDb) -> &dyn SymbolContext {
        todo!()
    }
}
