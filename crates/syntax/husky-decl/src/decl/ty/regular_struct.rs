use super::*;
use husky_token::{CommaToken, LeftCurlyBraceToken, RightCurlyBraceToken};
use parsec::parse_separated_list2;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct RegularStructTypeRawDecl {
    #[id]
    pub node_path: TypeNodePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    pub lcurl: LeftCurlyBraceToken,
    #[return_ref]
    field_comma_list: (Vec<RegularStructFieldDeclPattern>, Vec<CommaToken>),
    #[return_ref]
    pub rcurl: RightCurlyBraceToken,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct RegularStructTypeDecl {
    #[id]
    pub node_path: TypeNodePath,
    #[return_ref]
    implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    pub lcurl: LeftCurlyBraceToken,
    #[return_ref]
    field_comma_list: (Vec<RegularStructFieldDeclPattern>, Vec<CommaToken>),
    #[return_ref]
    pub rcurl: RightCurlyBraceToken,
    pub expr_region: ExprRegion,
}

impl RegularStructTypeDecl {
    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDeclPattern] {
        self.implicit_parameter_decl_list(db)
            .as_ref()
            .map(ImplicitParameterDeclList::implicit_parameters)
            .unwrap_or(&[])
    }

    pub fn fields<'a>(self, db: &'a dyn DeclDb) -> &'a [RegularStructFieldDeclPattern] {
        &self.field_comma_list(db).0
    }
}
