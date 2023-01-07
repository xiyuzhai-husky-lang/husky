use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct FunctionDefn {
    #[id]
    pub path: FormPath,
    pub decl: FunctionDecl,
    pub expr_sheet: ModuleItemDefnExprSheet,
    pub body: DefnResult<ExprIdx>,
}

impl FunctionDefn {
    pub fn symbol_context(self, db: &dyn DeclDb) -> &dyn SymbolContext {
        todo!()
    }
}
