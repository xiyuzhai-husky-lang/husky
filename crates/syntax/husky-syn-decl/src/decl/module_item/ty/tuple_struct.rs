use super::*;
use husky_syn_expr::SynExprIdx;
use parsec::{PunctuatedSmallList, TryParseFromStream};

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TupleStructTypeSynNodeDecl {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    #[return_ref]
    template_parameter_decl_list: SynNodeDeclResult<Option<Generics>>,
    lpar: LparRegionalToken,
    #[return_ref]
    field_comma_list: SynNodeDeclResult<
        PunctuatedSmallList<TupleFieldObelisk, CommaRegionalToken, 4, SynNodeDeclError>,
    >,
    #[return_ref]
    rpar: SynNodeDeclResult<TupleStructRparRegionalToken>,
    pub syn_expr_region: SynExprRegion,
}

impl TupleStructTypeSynNodeDecl {
    pub fn errors(self, db: &dyn SynDeclDb) -> SynNodeDeclErrorRefs {
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

impl<'a> TryParseFromStream<SynDeclExprParser<'a>> for TupleStructRparRegionalToken {
    type Error = SynNodeDeclError;

    fn try_parse_from_stream(sp: &mut SynDeclExprParser<'a>) -> Result<Self, Self::Error> {
        // todo: enrich this
        // consider unexpected
        // maybe sp.skip_exprs_until_next_right_parenthesis
        let rpar = sp.try_parse_expected(
            OriginalSynNodeDeclError::ExpectedRightParenthesisInTupleStructFieldTypeList,
        )?;
        Ok(Self(rpar))
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TupleStructTypeSynDecl {
    #[id]
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: ImplicitParameterDeclPatterns,
    #[return_ref]
    pub fields: SmallVec<[TupleFieldObelisk; 4]>,
    pub syn_expr_region: SynExprRegion,
}

impl TupleStructTypeSynDecl {
    #[inline(always)]
    pub(super) fn from_node_decl(
        db: &dyn SynDeclDb,
        path: TypePath,
        syn_node_decl: TupleStructTypeSynNodeDecl,
    ) -> DeclResult<Self> {
        let template_parameters = syn_node_decl
            .template_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.template_parameters().to_smallvec())
            .unwrap_or_default();
        let fields = SmallVec::from(syn_node_decl.field_comma_list(db).as_ref()?.elements());
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
