use super::*;
use husky_expr::ExprIdx;
use husky_word::Identifier;

#[salsa::tracked(jar = DeclJar)]
pub struct UnitStructTypeDecl {
    #[id]
    pub path: TypePath,
    pub ast_idx: AstIdx,
}
