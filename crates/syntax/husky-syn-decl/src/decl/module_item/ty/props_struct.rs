use super::*;
use husky_token::{CommaToken, LcurlToken, RcurlToken};
use parsec::{parse_separated_list2, PunctuatedSmallList, TryParseFromStream};

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct PropsStructTypeSynNodeDecl {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    template_parameter_decl_list: SynNodeDeclResult<Option<Generics>>,
    #[return_ref]
    lcurl: SynNodeDeclResult<PropsStructLeftCurlyBrace>,
    #[return_ref]
    fields: SynNodeDeclResult<
        PunctuatedSmallList<PropsFieldDeclPattern, CommaToken, 4, SynNodeDeclError>,
    >,
    #[return_ref]
    rcurl: SynNodeDeclResult<PropsStructRcurlToken>,
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
pub struct PropsStructLeftCurlyBrace(LcurlToken);

impl<'a, 'b> TryParseFromStream<ExprParseContext<'a, 'b>> for PropsStructLeftCurlyBrace {
    type Error = SynNodeDeclError;

    fn try_parse_from_stream(sp: &mut ExprParseContext) -> Result<Self, Self::Error> {
        let lcurl = sp.try_parse_expected(
            OriginalSynNodeDeclError::ExpectedLeftCurlyBraceOrLeftParenthesisOrSemicolonForStruct,
        )?;
        Ok(Self(lcurl))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropsStructRcurlToken(RcurlToken);

impl<'a, 'b> TryParseFromStream<ExprParseContext<'a, 'b>> for PropsStructRcurlToken {
    type Error = SynNodeDeclError;

    fn try_parse_from_stream(sp: &mut ExprParseContext) -> Result<Self, Self::Error> {
        // todo: enrich this
        // consider unexpected
        // maybe sp.skip_exprs_until_next_right_curly_brace
        let rcurl = sp.try_parse_expected(OriginalSynNodeDeclError::ExpectedRightCurlyBrace)?;
        Ok(Self(rcurl))
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct PropsStructTypeSynDecl {
    #[id]
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: ImplicitParameterDeclPatterns,
    #[return_ref]
    pub fields: SmallVec<[PropsFieldDeclPattern; 4]>,
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
            .map(|list| list.template_parameters().to_smallvec())
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
