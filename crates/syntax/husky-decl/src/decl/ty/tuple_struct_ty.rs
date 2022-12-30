use super::*;
use husky_expr::ExprIdx;
use husky_word::Identifier;

#[salsa::tracked(jar = DeclJar)]
pub struct TupleStructTypeDecl {
    #[id]
    pub path: TypePath,
    pub ast_idx: AstIdx,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TupleStructFieldDecl {
    ty: ExprIdx,
}
