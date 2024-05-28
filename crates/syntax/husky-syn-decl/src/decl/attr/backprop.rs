use super::*;
use husky_coword::Ident;
use parsec::{PunctuatedSmallList, TryParseOptionFromStream};

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar, constructor = new_inner)]
pub struct BackpropAttrSynNodeDecl {
    #[id]
    pub syn_node_path: AttrSynNodePath,
    pub pound_token: PoundRegionalToken,
    pub backprop_ident_token: IdentRegionalToken,
    #[return_ref]
    pub lpar_token: SynNodeDeclResult<LparRegionalToken>,
    #[return_ref]
    pub arguments: SynNodeDeclResult<
        PunctuatedSmallList<BackpropAttrArgument, CommaRegionalToken, SynNodeDeclError, true, 8>,
    >,
    #[return_ref]
    pub rpar_token: SynNodeDeclResult<RparRegionalToken>,
    #[skip_fmt]
    pub syn_expr_region: SynExprRegion,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BackpropAttrArgument {
    parameter_ident_token: IdentRegionalToken,
    eq_token: EqRegionalToken,
    backprop_function: SynExprIdx,
}

/// # constructor

impl BackpropAttrSynNodeDecl {
    pub(super) fn new(db: &::salsa::Db, syn_node_path: AttrSynNodePath) -> Self {
        let parser_factory = ItemDeclParser::new(db, syn_node_path.into());
        let mut parser = parser_factory.expr_parser(
            syn_node_path
                .parent_syn_node_path(db)
                .syn_node_decl(db)
                .syn_expr_region(db),
            AllowSelfType::True,
            AllowSelfValue::False,
            None,
        );
        let pound_token = parser
            .try_parse_option()
            .expect("should be guaranteed")
            .expect("should be guaranteed");
        let derive_token = parser
            .try_parse_option()
            .expect("should be guaranteed")
            .expect("should be guaranteed");
        let lpar_token =
            parser.try_parse_expected(OriginalSynNodeDeclError::ExpectedLeftDelimiterInDerive);
        let arguments = parser.try_parse();
        let rpar_token =
            parser.try_parse_expected(OriginalSynNodeDeclError::ExpectedRightDelimiterInDerive);
        Self::new_inner(
            db,
            syn_node_path,
            pound_token,
            derive_token,
            lpar_token,
            arguments,
            rpar_token,
            parser.finish(),
        )
    }
}

/// # getters

impl BackpropAttrSynNodeDecl {
    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        todo!()
    }
}

impl BackpropAttrArgument {
    pub fn parameter_ident(&self) -> Ident {
        self.parameter_ident_token.ident()
    }

    pub fn eq_token(&self) -> EqRegionalToken {
        self.eq_token
    }

    pub fn backprop_function(&self) -> SynExprIdx {
        self.backprop_function
    }
}

impl<'a> TryParseOptionFromStream<SynDeclExprParser<'a>> for BackpropAttrArgument {
    type Error = SynNodeDeclError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        sp: &mut SynDeclExprParser<'a>,
    ) -> SynNodeDeclResult<Option<Self>> {
        let Some(parameter_ident_token) = sp.try_parse_option::<IdentRegionalToken>()? else {
            return Ok(None);
        };
        let eq_token =
            sp.try_parse_expected(OriginalSynNodeDeclError::ExpectedEqTokenForBackpropArgument)?;
        let backprop_function = sp.parse_expr_expected(
            None,
            OriginalSynNodeDeclError::ExpectedExprForBackpropArgument,
        )?;
        Ok(Some(Self {
            parameter_ident_token,
            eq_token,
            backprop_function,
        }))
    }
}

/// # syn decl

#[salsa::tracked]
pub struct BackpropAttrSynDecl {
    #[id]
    pub path: AttrItemPath,
    #[return_ref]
    pub arguments: SmallVec<[BackpropAttrArgument; 8]>,
    pub syn_expr_region: SynExprRegion,
}

/// ## constructor

impl BackpropAttrSynDecl {
    #[inline(always)]
    pub(super) fn from_node_decl(
        db: &::salsa::Db,
        path: AttrItemPath,
        syn_node_decl: BackpropAttrSynNodeDecl,
    ) -> SynDeclResult<Self> {
        let trais = SmallVec::from(syn_node_decl.arguments(db).as_ref()?.elements());
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(Self::new(db, path, trais, syn_expr_region))
    }
}
