use super::*;
use husky_expr::ExprIdx;
use husky_token::{
    ColonToken, CommaToken, IdentifierToken, LeftCurlyBraceToken, RightCurlyBraceToken,
};
use husky_word::Identifier;
use parsec::ParseContext;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct RegularStructTypeDecl {
    #[id]
    pub path: TypePath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    pub implicit_parameter_decl_list: DeclExprResult<Option<ImplicitParameterDeclList>>,
    pub lcurl: LeftCurlyBraceToken,
    #[return_ref]
    pub field_comma_list: ExprResult<(Vec<RegularStructFieldPattern>, Vec<CommaToken>)>,
    #[return_ref]
    pub rcurl: DeclExprResult<RightCurlyBraceToken>,
}

impl RegularStructTypeDecl {
    pub fn implicit_parameters<'a>(
        self,
        db: &'a dyn DeclDb,
    ) -> DeclExprResultRef<'a, &'a [ImplicitParameterDecl]> {
        Ok(self
            .implicit_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|l| -> &[ImplicitParameterDecl] { &l })
            .unwrap_or(&[]))
    }

    pub fn fields<'a>(
        self,
        db: &'a dyn DeclDb,
    ) -> ExprResultRef<'a, &'a [RegularStructFieldPattern]> {
        Ok(self.field_comma_list(db).as_ref()?.0.as_ref())
    }
}
