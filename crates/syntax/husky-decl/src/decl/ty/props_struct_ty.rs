use super::*;
use husky_expr::ExprIdx;
use husky_word::Identifier;

#[salsa::tracked(jar = DeclJar)]
pub struct PropsStructTypeDecl {
    #[id]
    pub path: TypePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    pub fields: Vec<PropsStructFieldDecl>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PropsStructFieldDecl {
    ident: Identifier,
    ty: ExprIdx,
}
