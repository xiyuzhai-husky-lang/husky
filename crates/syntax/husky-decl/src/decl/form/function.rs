use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct FunctionDecl {
    #[id]
    pub path: FormPath,
    pub ast_idx: AstIdx,
    #[return_ref]
    pub implicit_parameters: Option<ImplicitParameterDeclList>,
}
