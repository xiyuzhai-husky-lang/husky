use super::*;
use husky_token::{CommaToken, LeftCurlyBraceToken, RightCurlyBraceToken};
use parsec::{parse_separated_list2, SeparatedSmallList, TryParseFromStream};

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct RegularStructTypeNodeDecl {
    #[id]
    pub node_path: TypeNodePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    implicit_parameter_decl_list: DeclExprResult<Option<ImplicitParameterDeclList>>,
    #[return_ref]
    lcurl: DeclExprResult<RegularStructLeftCurlyBrace>,
    #[return_ref]
    struct_fields: DeclExprResult<
        SeparatedSmallList<RegularStructFieldDeclPattern, CommaToken, 4, DeclExprError>,
    >,
    #[return_ref]
    rcurl: DeclExprResult<RegularStructRightCurlyBraceToken>,
    pub expr_region: ExprRegion,
}

impl RegularStructTypeNodeDecl {}

/// we delegate a struct for this for better error message
/// regular struct is the fallback case, but the lang user might want to mean other things
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegularStructLeftCurlyBrace(LeftCurlyBraceToken);

impl<'a, 'b> TryParseFromStream<ExprParseContext<'a, 'b>> for RegularStructLeftCurlyBrace {
    type Error = DeclExprError;

    fn try_parse_from_stream(sp: &mut ExprParseContext) -> Result<Self, Self::Error> {
        let lcurl = sp.try_parse_expected(
            OriginalDeclExprError::ExpectedLeftCurlyBraceOrLeftParenthesisOrSemicolonForStruct,
        )?;
        Ok(Self(lcurl))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegularStructRightCurlyBraceToken(RightCurlyBraceToken);

impl<'a, 'b> TryParseFromStream<ExprParseContext<'a, 'b>> for RegularStructRightCurlyBraceToken {
    type Error = DeclExprError;

    fn try_parse_from_stream(sp: &mut ExprParseContext) -> Result<Self, Self::Error> {
        // todo: enrich this
        // consider unexpected
        // maybe sp.skip_exprs_until_next_right_curly_brace
        let rcurl = sp.try_parse_expected(OriginalDeclExprError::ExpectedRightCurlyBrace)?;
        Ok(Self(rcurl))
    }
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct RegularStructTypeDecl {
    #[id]
    pub path: TypePath,
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclPatterns,
    #[return_ref]
    pub fields: SmallVec<[RegularStructFieldDeclPattern; 4]>,
    pub expr_region: ExprRegion,
}

impl RegularStructTypeDecl {
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: TypePath,
        node_decl: RegularStructTypeNodeDecl,
    ) -> DeclResult<Self> {
        let implicit_parameters = node_decl
            .implicit_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.implicit_parameters().to_smallvec())
            .unwrap_or_default();
        let fields = SmallVec::from(node_decl.struct_fields(db).as_ref()?.elements());
        let expr_region = node_decl.expr_region(db);
        Ok(Self::new(
            db,
            path,
            implicit_parameters,
            fields,
            expr_region,
        ))
    }
}
