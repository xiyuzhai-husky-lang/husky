use super::*;
use husky_regional_token::{CommaRegionalToken, LcurlRegionalToken, RcurlRegionalToken};
use parsec::{PunctuatedSmallList, TryParseFromStream};

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct PropsStructTypeSynNodeDecl {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    #[return_ref]
    template_parameter_decl_list: SynNodeDeclResult<Option<SynTemplateParameterSyndicateList>>,
    #[return_ref]
    lcurl: SynNodeDeclResult<PropsStructLcurlRegionalToken>,
    #[return_ref]
    fields: SynNodeDeclResult<
        PunctuatedSmallList<PropsFieldSyndicate, CommaRegionalToken, SynNodeDeclError, true, 4>,
    >,
    #[return_ref]
    rcurl: SynNodeDeclResult<PropsStructRcurlRegionalToken>,
    pub syn_expr_region: SynExprRegion,
}

impl PropsStructTypeSynNodeDecl {
    pub fn errors(self, db: &dyn SynDeclDb) -> SynNodeDeclErrorRefs {
        SmallVec::from_iter(
            self.template_parameter_decl_list(db)
                .as_ref()
                .err()
                .into_iter()
                .chain(self.lcurl(db).as_ref().err())
                .chain(self.fields(db).as_ref().err().into_iter())
                .chain(self.rcurl(db).as_ref().err().into_iter()),
        )
    }
}

/// we delegate a struct for this for better error message
/// regular struct is the fallback case, but the lang user might want to mean other things
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropsStructLcurlRegionalToken(LcurlRegionalToken);

impl<'a> TryParseFromStream<SynDeclExprParser<'a>> for PropsStructLcurlRegionalToken {
    type Error = SynNodeDeclError;

    fn try_parse_from_stream(sp: &mut SynDeclExprParser<'a>) -> Result<Self, Self::Error> {
        let lcurl = sp.try_parse_expected(
            OriginalSynNodeDeclError::ExpectedLcurlOrLparOrSemicolonForStruct,
        )?;
        Ok(Self(lcurl))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropsStructRcurlRegionalToken(RcurlRegionalToken);

impl<'a> TryParseFromStream<SynDeclExprParser<'a>> for PropsStructRcurlRegionalToken {
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
pub struct PropsStructTypeSynDecl {
    #[id]
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: TemplateParameterSyndicates,
    #[return_ref]
    pub fields: SmallVec<[PropsFieldSyndicate; 4]>,
    pub syn_expr_region: SynExprRegion,
}

impl PropsStructTypeSynDecl {
    pub(super) fn from_node_decl(
        db: &dyn SynDeclDb,
        path: TypePath,
        syn_node_decl: PropsStructTypeSynNodeDecl,
    ) -> DeclResult<Self> {
        let template_parameters = syn_node_decl
            .template_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.syn_template_parameter_obelisks().to_smallvec())
            .unwrap_or_default();
        let fields = SmallVec::from(syn_node_decl.fields(db).as_ref()?.elements());
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(Self::new(
            db,
            path,
            template_parameters,
            fields,
            syn_expr_region,
        ))
    }
}
