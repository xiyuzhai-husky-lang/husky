use super::*;

use husky_token::{
    CommaToken, LeftCurlyBraceToken, RightCurlyBraceToken,
};



#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct RegularStructTypeDecl {
    #[id]
    pub path: TypePath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    implicit_parameter_decl_list: DeclExprResult<Option<ImplicitParameterDeclList>>,
    pub lcurl: LeftCurlyBraceToken,
    #[return_ref]
    pub field_comma_list: (
        Vec<RegularStructFieldPattern>,
        Vec<CommaToken>,
        ExprResult<()>,
    ),
    #[return_ref]
    pub rcurl: DeclExprResult<RightCurlyBraceToken>,
}

impl RegularStructTypeDecl {
    pub fn implicit_parameters<'a>(
        self,
        db: &'a dyn DeclDb,
    ) -> DeclExprResultRef<'a, &'a [ImplicitParameterDecl]> {
        self.implicit_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(ImplicitParameterDeclList::implicit_parameters)
            .unwrap_or(Ok(&[]))
    }

    pub fn fields<'a>(
        self,
        db: &'a dyn DeclDb,
    ) -> ExprResultRef<'a, &'a [RegularStructFieldPattern]> {
        self.field_comma_list(db).2.as_ref()?;
        Ok(self.field_comma_list(db).0.as_ref())
    }
}
