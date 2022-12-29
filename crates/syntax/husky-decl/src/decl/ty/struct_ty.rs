use super::*;
use husky_expr::ExprIdx;
use husky_word::Identifier;

#[salsa::tracked(jar = DeclJar)]
pub struct StructTypeDecl {
    #[id]
    pub path: TypePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    pub fields: Vec<FieldDecl>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FieldDecl {
    ident: Identifier,
    ty: ExprIdx,
}
