use super::*;
use husky_syn_expr::ExprIdx;
use parsec::{SeparatedSmallList, TryParseFromStream};

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct TupleStructTypeSynNodeDecl {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    implicit_parameter_decl_list: NodeDeclResult<Option<Generics>>,
    lpar: LeftParenthesisToken,
    #[return_ref]
    field_comma_list:
        NodeDeclResult<SeparatedSmallList<TupleFieldDeclPattern, CommaToken, 4, NodeDeclError>>,
    #[return_ref]
    rpar: NodeDeclResult<TupleStructRightParenthesisToken>,
    pub expr_region: SynExprRegion,
}

impl TupleStructTypeSynNodeDecl {
    pub fn errors(self, db: &dyn DeclDb) -> NodeDeclErrorRefs {
        SmallVec::from_iter(
            self.implicit_parameter_decl_list(db)
                .as_ref()
                .err()
                .into_iter(),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TupleStructRightParenthesisToken(RightParenthesisToken);

impl<'a, 'b> TryParseFromStream<ExprParseContext<'a, 'b>> for TupleStructRightParenthesisToken {
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
pub struct TupleStructTypeSynDecl {
    #[id]
    pub path: TypePath,
    #[return_ref]
    pub generic_parameters: ImplicitParameterDeclPatterns,
    #[return_ref]
    pub fields: SmallVec<[TupleFieldDeclPattern; 4]>,
    pub expr_region: SynExprRegion,
}

impl TupleStructTypeSynDecl {
    #[inline(always)]
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: TypePath,
        syn_node_decl: TupleStructTypeSynNodeDecl,
    ) -> DeclResult<Self> {
        let generic_parameters = syn_node_decl
            .implicit_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.generic_parameters().to_smallvec())
            .unwrap_or_default();
        let fields = SmallVec::from(syn_node_decl.field_comma_list(db).as_ref()?.elements());
        let expr_region = syn_node_decl.expr_region(db);
        Ok(Self::new(db, path, generic_parameters, fields, expr_region))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TupleStructFieldDecl {
    ty: ExprIdx,
}
