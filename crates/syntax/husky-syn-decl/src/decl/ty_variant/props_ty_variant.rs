use super::*;
use husky_regional_token::{CommaRegionalToken, LcurlRegionalToken, RcurlRegionalToken};
use parsec::{PunctuatedSmallList, TryParseFromStream};

// todo: GADT
#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TypePropsVariantSynNodeDecl {
    #[id]
    pub syn_node_path: TypeVariantSynNodePath,
    #[return_ref]
    lcurl: SynNodeDeclResult<TypePropsVariantLeftCurlyBrace>,
    #[return_ref]
    fields: SynNodeDeclResult<
        PunctuatedSmallList<PropsFieldSyndicate, CommaRegionalToken, SynNodeDeclError, true, 4>,
    >,
    #[return_ref]
    rcurl: SynNodeDeclResult<TypePropsVariantRcurlRegionalToken>,
    pub syn_expr_region: SynExprRegion,
}

/// we delegate a struct for this for better error message
/// regular struct is the fallback case, but the lang user might want to mean other things
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypePropsVariantLeftCurlyBrace(LcurlRegionalToken);

impl<'a> TryParseFromStream<SynDeclExprParser<'a>> for TypePropsVariantLeftCurlyBrace {
    type Error = SynNodeDeclError;

    fn try_parse_from_stream(sp: &mut SynDeclExprParser<'a>) -> Result<Self, Self::Error> {
        let lcurl = sp.try_parse_expected(
            OriginalSynNodeDeclError::ExpectedLcurlOrLparOrSemicolonForStruct,
        )?;
        Ok(Self(lcurl))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypePropsVariantRcurlRegionalToken(RcurlRegionalToken);

impl<'a> TryParseFromStream<SynDeclExprParser<'a>> for TypePropsVariantRcurlRegionalToken {
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
pub struct TypePropsVariantSynDecl {
    #[id]
    pub path: TypeVariantPath,
    pub fields: SmallVec<[PropsFieldSyndicate; 4]>,
    pub syn_expr_region: SynExprRegion,
}

impl TypePropsVariantSynDecl {
    pub(super) fn from_node_decl(
        db: &::salsa::Db,
        path: TypeVariantPath,
        syn_node_decl: TypePropsVariantSynNodeDecl,
    ) -> DeclResult<Self> {
        let fields = SmallVec::from(syn_node_decl.fields(db).as_ref()?.elements());
        Ok(Self::new(
            db,
            path,
            fields,
            syn_node_decl.syn_expr_region(db),
        ))
    }
}
