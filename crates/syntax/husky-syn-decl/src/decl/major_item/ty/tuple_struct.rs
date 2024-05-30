use super::*;
use parsec::{PunctuatedSmallList, TryParseFromStream};

#[salsa::tracked(constructor = pub(super) new)]
pub struct TupleStructSynNodeDecl {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    #[return_ref]
    template_parameter_decl_list: SynNodeDeclResult<Option<SynTemplateParameterSyndicateList>>,
    lpar: LparRegionalToken,
    #[return_ref]
    field_comma_list: SynNodeDeclResult<
        PunctuatedSmallList<TupleFieldSyndicate, CommaRegionalToken, SynNodeDeclError, true, 4>,
    >,
    #[return_ref]
    rpar: SynNodeDeclResult<TupleStructRparRegionalToken>,
    pub syn_expr_region: SynExprRegion,
}

impl TupleStructSynNodeDecl {
    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        SmallVec::from_iter(
            self.template_parameter_decl_list(db)
                .as_ref()
                .err()
                .into_iter(),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TupleStructRparRegionalToken(RparRegionalToken);

impl<'a> TryParseFromStream<StandaloneSynExprParser<'a>> for TupleStructRparRegionalToken {
    type Error = SynNodeDeclError;

    fn try_parse_from_stream(sp: &mut StandaloneSynExprParser<'a>) -> Result<Self, Self::Error> {
        // todo: enrich this
        // consider unexpected
        // maybe sp.skip_exprs_until_next_right_parenthesis
        let rpar = sp.try_parse_expected(
            OriginalSynNodeDeclError::ExpectedRightParenthesisInTupleStructFieldTypeList,
        )?;
        Ok(Self(rpar))
    }
}

#[salsa::tracked]
pub struct TupleStructSynDecl {
    #[id]
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: TemplateSynParametersData,
    #[return_ref]
    pub fields: SmallVec<[TupleFieldSyndicate; 4]>,
    pub syn_expr_region: SynExprRegion,
}

impl TupleStructSynDecl {
    #[inline(always)]
    pub(super) fn from_node(
        db: &::salsa::Db,
        path: TypePath,
        syn_node_decl: TupleStructSynNodeDecl,
    ) -> SynDeclResult<Self> {
        let template_parameters = syn_node_decl
            .template_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| {
                list.syn_template_parameter_obelisks()
                    .iter()
                    .map(Clone::clone)
                    .collect()
            })
            .unwrap_or_default();
        let fields = SmallVec::from(syn_node_decl.field_comma_list(db).as_ref()?.elements());
        syn_node_decl.rpar(db).as_ref()?;
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

#[derive(Debug, PartialEq, Eq)]
pub struct TupleStructFieldDecl {
    ty: SynExprIdx,
}
