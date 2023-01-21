use super::*;
use husky_expr::ExprIdx;
use husky_token::{
    ColonToken, CommaToken, IdentifierToken, LeftCurlyBraceToken, RightCurlyBraceToken,
};
use husky_word::Identifier;
use parsec::ParseContext;

#[salsa::tracked(jar = DeclJar)]
pub struct RegularStructTypeDecl {
    #[id]
    pub path: TypePath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    pub implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    pub lcurl: LeftCurlyBraceToken,
    #[return_ref]
    pub fields: Vec<RegularStructFieldDecl>,
    #[return_ref]
    pub separators: Vec<CommaToken>,
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

#[derive(Debug, PartialEq, Eq)]
pub struct RegularStructFieldDecl {
    ident_token: IdentifierToken,
    colon: ColonToken,
    ty: ExprIdx,
}

impl RegularStructFieldDecl {
    pub fn ident(&self) -> Identifier {
        self.ident_token.ident()
    }

    pub fn colon(&self) -> ColonToken {
        self.colon
    }

    pub fn ty(&self) -> ExprIdx {
        self.ty
    }
}

impl<'a, 'b> parsec::ParseFrom<ExprParseContext<'a, 'b>> for RegularStructFieldDecl {
    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, ExprError> {
        let Some(ident) = ctx.parse::<IdentifierToken>()? else {
                return Ok(None)
            };
        let colon: ColonToken = ctx.parse_expected()?;
        let Some(expr) = ctx.parse_expr(ExprParseEnvironment::None) else { todo!() };
        Ok(Some(RegularStructFieldDecl {
            ident_token: ident,
            colon,
            ty: expr,
        }))
    }
}
