use super::*;
use husky_token::{CommaToken, LeftCurlyBraceToken, RightCurlyBraceToken};
use parsec::{parse_separated_list2, SeparatedSmallList, TryParseFromStream};

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct PropsStructTypeNodeDecl {
    #[id]
    pub node_path: TypeNodePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    implicit_parameter_decl_list: NodeDeclResult<Option<ImplicitParameterDeclList>>,
    #[return_ref]
    lcurl: NodeDeclResult<PropsStructLeftCurlyBrace>,
    #[return_ref]
    fields: NodeDeclResult<SeparatedSmallList<PropsFieldDeclPattern, CommaToken, 4, NodeDeclError>>,
    #[return_ref]
    rcurl: NodeDeclResult<PropsStructRightCurlyBraceToken>,
    pub expr_region: ExprRegion,
}

impl PropsStructTypeNodeDecl {
    pub fn errors(self, db: &dyn DeclDb) -> NodeDeclErrorRefs {
        SmallVec::from_iter(
            self.implicit_parameter_decl_list(db)
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
pub struct PropsStructLeftCurlyBrace(LeftCurlyBraceToken);

impl<'a, 'b> TryParseFromStream<ExprParseContext<'a, 'b>> for PropsStructLeftCurlyBrace {
    type Error = NodeDeclError;

    fn try_parse_from_stream(sp: &mut ExprParseContext) -> Result<Self, Self::Error> {
        let lcurl = sp.try_parse_expected(
            OriginalNodeDeclError::ExpectedLeftCurlyBraceOrLeftParenthesisOrSemicolonForStruct,
        )?;
        Ok(Self(lcurl))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropsStructRightCurlyBraceToken(RightCurlyBraceToken);

impl<'a, 'b> TryParseFromStream<ExprParseContext<'a, 'b>> for PropsStructRightCurlyBraceToken {
    type Error = NodeDeclError;

    fn try_parse_from_stream(sp: &mut ExprParseContext) -> Result<Self, Self::Error> {
        // todo: enrich this
        // consider unexpected
        // maybe sp.skip_exprs_until_next_right_curly_brace
        let rcurl = sp.try_parse_expected(OriginalNodeDeclError::ExpectedRightCurlyBrace)?;
        Ok(Self(rcurl))
    }
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct PropsStructTypeDecl {
    #[id]
    pub path: TypePath,
    #[return_ref]
    pub generic_parameters: ImplicitParameterDeclPatterns,
    #[return_ref]
    pub fields: SmallVec<[PropsFieldDeclPattern; 4]>,
    pub expr_region: ExprRegion,
}

impl PropsStructTypeDecl {
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: TypePath,
        node_decl: PropsStructTypeNodeDecl,
    ) -> DeclResult<Self> {
        let generic_parameters = node_decl
            .implicit_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.generic_parameters().to_smallvec())
            .unwrap_or_default();
        let fields = SmallVec::from(node_decl.fields(db).as_ref()?.elements());
        let expr_region = node_decl.expr_region(db);
        Ok(Self::new(db, path, generic_parameters, fields, expr_region))
    }
}
