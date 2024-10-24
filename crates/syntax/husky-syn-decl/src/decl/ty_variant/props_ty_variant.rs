use super::*;
use parsec::{PunctuatedSmallList, TryParseFromStream};

// todo: GADT
#[salsa::tracked]
pub struct TypePropsVariantSynNodeDecl {
    #[id]
    pub syn_node_path: TypeVariantSynNodePath,
    lcurl_token_idx: RegionalTokenIdx,
    #[return_ref]
    fields: SynNodeDeclResult<
        PunctuatedSmallList<PropsFieldSyndicate, CommaRegionalToken, SynNodeDeclError, true, 4>,
    >,
    #[return_ref]
    rcurl: SynNodeDeclResult<TypePropsVariantRcurlRegionalToken>,
    pub syn_expr_region: SynExprRegion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypePropsVariantRcurlRegionalToken(InlineRcurlRegionalToken);

impl<'a> TryParseFromStream<StandaloneSynExprParser<'a>> for TypePropsVariantRcurlRegionalToken {
    type Error = SynNodeDeclError;

    fn try_parse_from_stream(sp: &mut StandaloneSynExprParser<'a>) -> Result<Self, Self::Error> {
        // todo: enrich this
        // consider unexpected
        // maybe sp.skip_exprs_until_next_right_curly_brace
        let rcurl =
            sp.try_parse_expected(OriginalSynNodeDeclError::ExpectedRcurlForTypePropsVariant)?;
        Ok(Self(rcurl))
    }
}

/// # getters
impl TypePropsVariantSynNodeDecl {
    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        SmallVec::from_iter(
            self.fields(db)
                .as_ref()
                .err()
                .into_iter()
                .chain(self.rcurl(db).as_ref().err().into_iter()),
        )
    }
}

#[salsa::tracked]
pub struct TypePropsVariantSynDecl {
    #[id]
    pub path: TypeVariantPath,
    pub fields: SmallVec<[PropsFieldSyndicate; 4]>,
    pub syn_expr_region: SynExprRegion,
}

impl TypePropsVariantSynDecl {
    pub(super) fn from_node(
        db: &::salsa::Db,
        path: TypeVariantPath,
        syn_node_decl: TypePropsVariantSynNodeDecl,
    ) -> SynDeclResult<Self> {
        let fields = SmallVec::from(syn_node_decl.fields(db).as_ref()?.elements());
        syn_node_decl.rcurl(db).as_ref()?;
        Ok(Self::new(
            db,
            path,
            fields,
            syn_node_decl.syn_expr_region(db),
        ))
    }
}
