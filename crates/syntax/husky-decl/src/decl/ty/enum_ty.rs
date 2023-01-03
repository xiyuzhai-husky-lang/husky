use super::*;

#[salsa::tracked(jar = DeclJar)]
pub struct EnumTypeDecl {
    #[id]
    pub path: TypePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    pub expr_sheet: ExprSheet,
    #[return_ref]
    pub implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
}

impl EnumTypeDecl {
    pub fn implicit_parameters(self, db: &dyn DeclDb) -> &[ImplicitParameterDecl] {
        self.implicit_parameter_decl_list(db)
            .as_ref()
            .map(|l| -> &[ImplicitParameterDecl] { &l })
            .unwrap_or(&[])
    }
}
