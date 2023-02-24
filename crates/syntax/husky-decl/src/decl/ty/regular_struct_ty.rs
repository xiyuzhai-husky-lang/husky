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
    pub implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    pub lcurl: LeftCurlyBraceToken,
    #[return_ref]
    pub fields: ExprResult<(Vec<RegularStructFieldPattern>, Vec<CommaToken>)>,
    pub rcurl: RightCurlyBraceToken,
}

impl RegularStructTypeDecl {
    pub fn implicit_parameters(self, db: &dyn DeclDb) -> &[ImplicitParameterDecl] {
        self.implicit_parameter_decl_list(db)
            .as_ref()
            .map(|l| -> &[ImplicitParameterDecl] { &l })
            .unwrap_or(&[])
    }
}
