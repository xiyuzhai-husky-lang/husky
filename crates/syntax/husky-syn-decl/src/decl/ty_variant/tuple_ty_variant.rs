use super::*;
use husky_syn_expr::ExprIdx;
use parsec::{SeparatedSmallList, TryParseFromStream};

// todo: GADT
#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct TupleTypeVariantSynNodeDecl {
    #[id]
    pub syn_node_path: TypeVariantSynNodePath,
    pub ast_idx: AstIdx,
    lpar_token_idx: TokenIdx,
    #[return_ref]
    field_comma_list:
        NodeDeclResult<SeparatedSmallList<TupleFieldDeclPattern, CommaToken, 4, NodeDeclError>>,
    #[return_ref]
    rpar: NodeDeclResult<TupleTypeVariantRightParenthesisToken>,
    pub expr_region: SynExprRegion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TupleTypeVariantRightParenthesisToken(RightParenthesisToken);

impl<'a, 'b> TryParseFromStream<ExprParseContext<'a, 'b>>
    for TupleTypeVariantRightParenthesisToken
{
    type Error = NodeDeclError;

    fn try_parse_from_stream(sp: &mut ExprParseContext) -> Result<Self, Self::Error> {
        // todo: enrich this
        // consider unexpected
        // maybe sp.skip_exprs_until_next_right_parenthesis
        let rpar = sp.try_parse_expected(
            OriginalNodeDeclError::ExpectedRightParenthesisInTupleStructFieldTypeList,
        )?;
        Ok(Self(rpar))
    }
}

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct TupleTypeVariantSynDecl {
    #[id]
    pub path: TypeVariantPath,
    pub fields: SmallVec<[TupleFieldDeclPattern; 4]>,
    pub expr_region: SynExprRegion,
}

impl TupleTypeVariantSynDecl {
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: TypeVariantPath,
        syn_node_decl: TupleTypeVariantSynNodeDecl,
    ) -> DeclResult<Self> {
        let fields = SmallVec::from(syn_node_decl.field_comma_list(db).as_ref()?.elements());
        Ok(Self::new(db, path, fields, syn_node_decl.expr_region(db)))
    }
}
