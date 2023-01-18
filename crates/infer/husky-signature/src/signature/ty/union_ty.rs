use super::*;

#[salsa::interned(jar = SignatureJar)]
pub struct UnionTypeSignature {
    #[id]
    pub path: TypePath,
    pub ast_idx: AstIdx,
    pub expr_page: ExprPage,
    #[return_ref]
    pub implicit_parameter_decl_list: Option<ImplicitParameterSignatureList>,
}

impl UnionTypeSignature {
    pub fn implicit_parameters(self, db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        self.implicit_parameter_decl_list(db)
            .as_ref()
            .map(|l| -> &[ImplicitParameterSignature] { &l })
            .unwrap_or(&[])
    }
}
