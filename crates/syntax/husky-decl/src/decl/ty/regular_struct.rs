use super::*;

use husky_token::{CommaToken, LeftCurlyBraceToken, RightCurlyBraceToken};
use parsec::parse_separated_list2;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct RegularStructTypeDecl {
    #[id]
    pub path: TypePath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    pub lcurl: LeftCurlyBraceToken,
    #[return_ref]
    field_comma_list: (Vec<FieldDeclPattern>, Vec<CommaToken>),
    #[return_ref]
    pub rcurl: RightCurlyBraceToken,
}

impl RegularStructTypeDecl {
    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDecl] {
        self.implicit_parameter_decl_list(db)
            .as_ref()
            .map(ImplicitParameterDeclList::implicit_parameters)
            .unwrap_or(&[])
    }

    pub fn fields<'a>(self, db: &'a dyn DeclDb) -> &'a [FieldDeclPattern] {
        &self.field_comma_list(db).0
    }
}
