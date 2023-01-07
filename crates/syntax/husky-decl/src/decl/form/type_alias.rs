use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct TypeAliasDecl {
    #[id]
    pub path: FormPath,
    pub ast_idx: AstIdx,
    pub expr_sheet: ExprSheet,
}
