use super::*;
use husky_syn_expr::SynExprIdx;
use parsec::{PunctuatedSmallList, TryParseFromStream};

// todo: GADT
#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TupleTypeVariantSynNodeDecl {
    #[id]
    pub syn_node_path: TypeVariantSynNodePath,
    lpar_token_idx: RegionalTokenIdx,
    #[return_ref]
    field_comma_list: SynNodeDeclResult<
        PunctuatedSmallList<TupleFieldSyndicate, CommaRegionalToken, SynNodeDeclError, true, 4>,
    >,
    #[return_ref]
    rpar: SynNodeDeclResult<TupleTypeVariantRparRegionalToken>,
    pub syn_expr_region: SynExprRegion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TupleTypeVariantRparRegionalToken(RparRegionalToken);

impl<'a> TryParseFromStream<SynDeclExprParser<'a>> for TupleTypeVariantRparRegionalToken {
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
pub struct TupleTypeVariantSynDecl {
    #[id]
    pub path: TypeVariantPath,
    pub fields: SmallVec<[TupleFieldSyndicate; 4]>,
    pub syn_expr_region: SynExprRegion,
}

impl TupleTypeVariantSynDecl {
    pub(super) fn from_node_decl(
        db: &dyn SynDeclDb,
        path: TypeVariantPath,
        syn_node_decl: TupleTypeVariantSynNodeDecl,
    ) -> DeclResult<Self> {
        let fields = SmallVec::from(syn_node_decl.field_comma_list(db).as_ref()?.elements());
        Ok(Self::new(
            db,
            path,
            fields,
            syn_node_decl.syn_expr_region(db),
        ))
    }
}
