use super::*;
use husky_expr::ExprIdx;
use husky_word::Identifier;

#[salsa::tracked(jar = DeclJar)]
pub struct TupleStructTypeDecl {
    #[id]
    pub path: TypePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    pub generic_parameters: Vec<GenericParameter>,
    #[return_ref]
    pub fields: Vec<TupleStructFieldDecl>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TupleStructFieldDecl {
    ty: ExprIdx,
}
