use super::*;
use husky_token::{CommaToken, LeftCurlyBraceToken, RightCurlyBraceToken};
use parsec::{parse_separated_list2, SeparatedSmallList, TryParseFromStream};

// todo: GADT
#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct PropsTypeVariantSynNodeDecl {
    #[id]
    pub syn_node_path: TypeVariantSynNodePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    lcurl: NodeDeclResult<PropsTypeVariantLeftCurlyBrace>,
    #[return_ref]
    fields: NodeDeclResult<SeparatedSmallList<PropsFieldDeclPattern, CommaToken, 4, NodeDeclError>>,
    #[return_ref]
    rcurl: NodeDeclResult<PropsTypeVariantRightCurlyBraceToken>,
    pub expr_region: SynExprRegion,
}

/// we delegate a struct for this for better error message
/// regular struct is the fallback case, but the lang user might want to mean other things
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropsTypeVariantLeftCurlyBrace(LeftCurlyBraceToken);

impl<'a, 'b> TryParseFromStream<ExprParseContext<'a, 'b>> for PropsTypeVariantLeftCurlyBrace {
    type Error = NodeDeclError;

    fn try_parse_from_stream(sp: &mut ExprParseContext) -> Result<Self, Self::Error> {
        let lcurl = sp.try_parse_expected(
            OriginalNodeDeclError::ExpectedLeftCurlyBraceOrLeftParenthesisOrSemicolonForStruct,
        )?;
        Ok(Self(lcurl))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropsTypeVariantRightCurlyBraceToken(RightCurlyBraceToken);

impl<'a, 'b> TryParseFromStream<ExprParseContext<'a, 'b>> for PropsTypeVariantRightCurlyBraceToken {
    type Error = NodeDeclError;

    fn try_parse_from_stream(sp: &mut ExprParseContext) -> Result<Self, Self::Error> {
        // todo: enrich this
        // consider unexpected
        // maybe sp.skip_exprs_until_next_right_curly_brace
        let rcurl = sp.try_parse_expected(OriginalNodeDeclError::ExpectedRightCurlyBrace)?;
        Ok(Self(rcurl))
    }
}

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct PropsTypeVariantSynDecl {
    #[id]
    pub path: TypeVariantPath,
    pub expr_region: SynExprRegion,
}

impl PropsTypeVariantSynDecl {
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: TypeVariantPath,
        syn_node_decl: PropsTypeVariantSynNodeDecl,
    ) -> DeclResult<Self> {
        Ok(Self::new(db, path, syn_node_decl.expr_region(db)))
    }
}
