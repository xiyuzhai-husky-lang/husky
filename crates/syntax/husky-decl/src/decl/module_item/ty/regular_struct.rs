use super::*;
use husky_token::{CommaToken, LeftCurlyBraceToken, RightCurlyBraceToken};
use parsec::{parse_separated_list2, SeparatedListWithKet};

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct RegularStructTypeNodeDecl {
    #[id]
    pub node_path: TypeNodePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    implicit_parameter_decl_list: DeclExprResult<Option<ImplicitParameterDeclList>>,
    #[return_ref]
    struct_fields: SeparatedListWithKet<
        RegularStructFieldDeclPattern,
        CommaToken,
        RightCurlyBraceToken,
        DeclExprError,
    >,
    pub expr_region: ExprRegion,
}

impl RegularStructTypeNodeDecl {
    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDeclPattern] {
        todo!()
        // self.implicit_parameter_decl_list(db)
        //     .as_ref()
        //     .map(ImplicitParameterDeclList::implicit_parameters)
        //     .unwrap_or(&[])
    }

    pub fn fields<'a>(self, db: &'a dyn DeclDb) -> &'a [RegularStructFieldDeclPattern] {
        todo!()
        // &self.field_comma_list(db).0
    }
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct RegularStructTypeDecl {
    #[id]
    pub path: TypePath,
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclPatterns,
    pub lcurl: LeftCurlyBraceToken,
    #[return_ref]
    field_comma_list: (Vec<RegularStructFieldDeclPattern>, Vec<CommaToken>),
    #[return_ref]
    pub rcurl: RightCurlyBraceToken,
    pub expr_region: ExprRegion,
}

impl RegularStructTypeDecl {
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: TypePath,
        node_decl: RegularStructTypeNodeDecl,
    ) -> DeclResult<Self> {
        todo!()
    }

    pub fn fields<'a>(self, db: &'a dyn DeclDb) -> &'a [RegularStructFieldDeclPattern] {
        &self.field_comma_list(db).0
    }
}
