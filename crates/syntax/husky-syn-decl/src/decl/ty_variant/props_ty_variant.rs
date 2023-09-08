use super::*;
use husky_token::{CommaToken, LcurlToken, RcurlToken};
use parsec::{parse_separated_list2, PunctuatedSmallList, TryParseFromStream};

// todo: GADT
#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct PropsTypeVariantSynNodeDecl {
    #[id]
    pub syn_node_path: TypeVariantSynNodePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    lcurl: SynNodeDeclResult<PropsTypeVariantLeftCurlyBrace>,
    #[return_ref]
    fields: SynNodeDeclResult<
        PunctuatedSmallList<PropsFieldDeclPattern, CommaToken, 4, SynNodeDeclError>,
    >,
    #[return_ref]
    rcurl: SynNodeDeclResult<PropsTypeVariantRcurlToken>,
    pub syn_expr_region: SynExprRegion,
}

/// we delegate a struct for this for better error message
/// regular struct is the fallback case, but the lang user might want to mean other things
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropsTypeVariantLeftCurlyBrace(LcurlToken);

impl<'a> TryParseFromStream<SynDeclExprParser<'a>> for PropsTypeVariantLeftCurlyBrace {
    type Error = SynNodeDeclError;

    fn try_parse_from_stream(sp: &mut SynDeclExprParser<'a>) -> Result<Self, Self::Error> {
        let lcurl = sp.try_parse_expected(
            OriginalSynNodeDeclError::ExpectedLeftCurlyBraceOrLeftParenthesisOrSemicolonForStruct,
        )?;
        Ok(Self(lcurl))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropsTypeVariantRcurlToken(RcurlToken);

impl<'a> TryParseFromStream<SynDeclExprParser<'a>> for PropsTypeVariantRcurlToken {
    type Error = SynNodeDeclError;

    fn try_parse_from_stream(sp: &mut SynDeclExprParser<'a>) -> Result<Self, Self::Error> {
        // todo: enrich this
        // consider unexpected
        // maybe sp.skip_exprs_until_next_right_curly_brace
        let rcurl = sp.try_parse_expected(OriginalSynNodeDeclError::ExpectedRightCurlyBrace)?;
        Ok(Self(rcurl))
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct PropsTypeVariantSynDecl {
    #[id]
    pub path: TypeVariantPath,
    pub syn_expr_region: SynExprRegion,
}

impl PropsTypeVariantSynDecl {
    pub(super) fn from_node_decl(
        db: &dyn SynDeclDb,
        path: TypeVariantPath,
        syn_node_decl: PropsTypeVariantSynNodeDecl,
    ) -> DeclResult<Self> {
        Ok(Self::new(db, path, syn_node_decl.syn_expr_region(db)))
    }
}
