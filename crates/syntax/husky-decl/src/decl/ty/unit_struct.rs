use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct UnitStructTypeRawDecl {
    #[id]
    pub node_path: TypeNodePath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct UnitStructTypeDecl {
    #[id]
    pub node_path: TypeNodePath,
    pub expr_region: ExprRegion,
    #[return_ref]
    implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
}

impl UnitStructTypeDecl {
    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDeclPattern] {
        self.implicit_parameter_decl_list(db)
            .as_ref()
            .map(ImplicitParameterDeclList::implicit_parameters)
            .unwrap_or(&[])
    }
}
