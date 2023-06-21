use super::*;
use husky_expr::ExprIdx;
use parsec::{SeparatedSmallList, TryParseFromStream};

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TupleStructTypeNodeDecl {
    #[id]
    pub node_path: TypeNodePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    implicit_parameter_decl_list: DeclExprResult<Option<ImplicitParameterDeclList>>,
    #[return_ref]
    lpar: LeftParenthesisToken,
    #[return_ref]
    field_comma_list: DeclExprResult<
        SeparatedSmallList<TupleStructFieldDeclPattern, CommaToken, 4, DeclExprError>,
    >,
    #[return_ref]
    rpar: DeclExprResult<TupleStructRightParenthesisToken>,
    pub expr_region: ExprRegion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TupleStructRightParenthesisToken(RightParenthesisToken);

impl<'a, 'b> TryParseFromStream<ExprParseContext<'a, 'b>> for TupleStructRightParenthesisToken {
    type Error = DeclExprError;

    fn try_parse_from_stream(sp: &mut ExprParseContext) -> Result<Self, Self::Error> {
        // todo: enrich this
        // consider unexpected
        // maybe sp.skip_exprs_until_next_right_parenthesis
        let rpar = sp.try_parse_expected(
            OriginalDeclExprError::ExpectedRightParenthesisInTupleStructFieldTypeList,
        )?;
        Ok(Self(rpar))
    }
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TupleStructTypeDecl {
    #[id]
    pub path: TypePath,
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclPatterns,
    pub lpar: LeftParenthesisToken,
    #[return_ref]
    field_comma_list: (Vec<TupleStructFieldDeclPattern>, Vec<CommaToken>),
    pub rpar: RightParenthesisToken,
    pub expr_region: ExprRegion,
}

impl TupleStructTypeDecl {
    #[inline(always)]
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: TypePath,
        node_decl: TupleStructTypeNodeDecl,
    ) -> DeclResult<Self> {
        todo!()
    }

    pub fn fields<'a>(self, db: &'a dyn DeclDb) -> &'a [TupleStructFieldDeclPattern] {
        &self.field_comma_list(db).0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TupleStructFieldDecl {
    ty: ExprIdx,
}
