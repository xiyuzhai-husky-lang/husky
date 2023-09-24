use super::*;
use husky_regional_token::{CommaRegionalToken, LcurlRegionalToken, RcurlRegionalToken};
use parsec::{parse_separated_list2, PunctuatedSmallList, TryParseFromStream};

// todo: GADT
#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct PropsTypeVariantSynNodeDecl {
    #[id]
    pub syn_node_path: TypeVariantSynNodePath,
    #[return_ref]
    lcurl: SynNodeDeclResult<PropsTypeVariantLeftCurlyBrace>,
    #[return_ref]
    fields: SynNodeDeclResult<
        PunctuatedSmallList<PropsFieldDeclPattern, CommaRegionalToken, SynNodeDeclError, true, 4>,
    >,
    #[return_ref]
    rcurl: SynNodeDeclResult<PropsTypeVariantRcurlRegionalToken>,
    pub syn_expr_region: SynExprRegion,
}

/// we delegate a struct for this for better error message
/// regular struct is the fallback case, but the lang user might want to mean other things
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropsTypeVariantLeftCurlyBrace(LcurlRegionalToken);

impl<'a> TryParseFromStream<SynDeclExprParser<'a>> for PropsTypeVariantLeftCurlyBrace {
    type Error = SynNodeDeclError;

    fn try_parse_from_stream(sp: &mut SynDeclExprParser<'a>) -> Result<Self, Self::Error> {
        let lcurl = sp.try_parse_expected(
            OriginalSynNodeDeclError::ExpectedLcurlOrLparOrSemicolonForStruct,
        )?;
        Ok(Self(lcurl))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropsTypeVariantRcurlRegionalToken(RcurlRegionalToken);

impl<'a> TryParseFromStream<SynDeclExprParser<'a>> for PropsTypeVariantRcurlRegionalToken {
    type Error = SynNodeDeclError;

    fn try_parse_from_stream(sp: &mut SynDeclExprParser<'a>) -> Result<Self, Self::Error> {
        // todo: enrich this
        // consider unexpected
        // maybe sp.skip_exprs_until_next_right_curly_brace
        let rcurl = sp.try_parse_expected(OriginalSynNodeDeclError::ExpectedRcurl)?;
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
