use super::*;
use husky_expr::ExprIdx;
use husky_word::Identifier;

#[salsa::interned(jar = SignatureJar)]
pub struct TupleStructTypeSignature {
    #[id]
    pub path: TypePath,
    pub ast_idx: AstIdx,
    pub expr_page: ExprPage,
    #[return_ref]
    pub implicit_parameter_decl_list: Option<ImplicitParameterSignatureList>,
    #[return_ref]
    pub fields: Vec<TupleStructFieldSignature>,
}

impl TupleStructTypeSignature {
    pub fn implicit_parameters(self, db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        self.implicit_parameter_decl_list(db)
            .as_ref()
            .map(|l| -> &[ImplicitParameterSignature] { &l })
            .unwrap_or(&[])
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TupleStructFieldSignature {
    ty: ExprIdx,
}
