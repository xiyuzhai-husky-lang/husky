use super::*;
use husky_token::{CommaToken, LeftCurlyBraceToken, RightCurlyBraceToken};
use parsec::{parse_separated_list2, SeparatedSmallList, TryParseFromStream};

// todo: GADT
#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct PropsTypeVariantNodeDecl {
    #[id]
    pub node_path: TypeVariantNodePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    lcurl: DeclExprResult<PropsTypeVariantLeftCurlyBrace>,
    #[return_ref]
    fields: DeclExprResult<SeparatedSmallList<PropsFieldDeclPattern, CommaToken, 4, DeclExprError>>,
    #[return_ref]
    rcurl: DeclExprResult<PropsTypeVariantRightCurlyBraceToken>,
    pub expr_region: ExprRegion,
}

/// we delegate a struct for this for better error message
/// regular struct is the fallback case, but the lang user might want to mean other things
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropsTypeVariantLeftCurlyBrace(LeftCurlyBraceToken);

impl<'a, 'b> TryParseFromStream<ExprParseContext<'a, 'b>> for PropsTypeVariantLeftCurlyBrace {
    type Error = DeclExprError;

    fn try_parse_from_stream(sp: &mut ExprParseContext) -> Result<Self, Self::Error> {
        let lcurl = sp.try_parse_expected(
            OriginalDeclExprError::ExpectedLeftCurlyBraceOrLeftParenthesisOrSemicolonForStruct,
        )?;
        Ok(Self(lcurl))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropsTypeVariantRightCurlyBraceToken(RightCurlyBraceToken);

impl<'a, 'b> TryParseFromStream<ExprParseContext<'a, 'b>> for PropsTypeVariantRightCurlyBraceToken {
    type Error = DeclExprError;

    fn try_parse_from_stream(sp: &mut ExprParseContext) -> Result<Self, Self::Error> {
        // todo: enrich this
        // consider unexpected
        // maybe sp.skip_exprs_until_next_right_curly_brace
        let rcurl = sp.try_parse_expected(OriginalDeclExprError::ExpectedRightCurlyBrace)?;
        Ok(Self(rcurl))
    }
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct PropsTypeVariantDecl {
    #[id]
    pub path: TypeVariantPath,
    pub expr_region: ExprRegion,
}

impl PropsTypeVariantDecl {
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: TypeVariantPath,
        node_decl: PropsTypeVariantNodeDecl,
    ) -> DeclResult<Self> {
        Ok(Self::new(db, path, node_decl.expr_region(db)))
    }
}
