use husky_token::{CurryToken, EolColonToken};

use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct FunctionDecl {
    #[id]
    pub path: FormPath,
    pub ast_idx: AstIdx,
    pub expr_page: ExprPage,
    #[return_ref]
    pub implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    #[return_ref]
    pub parameter_decl_list: ParameterDeclList,
    #[return_ref]
    pub curry_token: DeclResult<CurryToken>,
    #[return_ref]
    pub output_ty: DeclResult<ExprIdx>,
    #[return_ref]
    pub eol_colon: DeclResult<EolColonToken>,
}

impl FunctionDecl {
    pub fn implicit_parameters(self, db: &dyn DeclDb) -> &[ImplicitParameterDecl] {
        self.implicit_parameter_decl_list(db)
            .as_ref()
            .map(|l| -> &[ImplicitParameterDecl] { &l })
            .unwrap_or(&[])
    }
}
