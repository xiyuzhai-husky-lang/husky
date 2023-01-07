use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct MorphismDecl {
    #[id]
    pub path: FormPath,
    pub ast_idx: AstIdx,
    pub expr_sheet: ModuleItemDeclExprSheet,
    #[return_ref]
    pub implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
}

impl MorphismDecl {
    pub fn implicit_parameters(self, db: &dyn DeclDb) -> &[ImplicitParameterDecl] {
        self.implicit_parameter_decl_list(db)
            .as_ref()
            .map(|l| -> &[ImplicitParameterDecl] { &l })
            .unwrap_or(&[])
    }
}
