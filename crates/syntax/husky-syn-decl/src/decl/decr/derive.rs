use parsec::{parse_separated_list2, PunctuatedSmallList};

use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar, constructor = new_inner)]
pub struct DeriveDecrSynNodeDecl {
    #[id]
    pub syn_node_path: DecrSynNodePath,
    pub ast_idx: AstIdx,
    pub at_token: AtToken,
    pub derive_token: IdentToken,
    #[return_ref]
    pub lpar_token: SynNodeDeclResult<RegionalLparToken>,
    #[return_ref]
    pub trais:
        SynNodeDeclResult<PunctuatedSmallList<TraitObelisk, CommaToken, 8, SynNodeDeclError>>,
    #[return_ref]
    pub rpar_token: SynNodeDeclResult<RparToken>,
    pub syn_expr_region: SynExprRegion,
}

impl DeriveDecrSynNodeDecl {
    pub(super) fn new(db: &dyn SynDeclDb, syn_node_path: DecrSynNodePath, ast_idx: AstIdx) -> Self {
        let parser_factory = DeclParserFactory::new(db, syn_node_path);
        let Ast::Decr {
            token_group_idx,
            ident,
        } = parser_factory.ast_sheet()[ast_idx]
        else {
            unreachable!()
        };
        let mut parser = parser_factory.parser(
            syn_node_path,
            syn_node_path
                .parent_syn_node_path(db)
                .syn_node_decl(db)
                .syn_expr_region(db),
            AllowSelfType::True,
            AllowSelfValue::False,
            None,
            token_group_idx,
            None,
        );
        let at_token = parser
            .try_parse_option()
            .expect("should be guaranteed")
            .expect("should be guaranteed");
        let derive_token = parser
            .try_parse_option()
            .expect("should be guaranteed")
            .expect("should be guaranteed");
        let lpar_token =
            parser.try_parse_expected(OriginalSynNodeDeclError::ExpectLeftBracketInDerive);
        let trais = parser.try_parse();
        let rpar_token =
            parser.try_parse_expected(OriginalSynNodeDeclError::ExpectRightBracketInDerive);
        Self::new_inner(
            db,
            syn_node_path,
            ast_idx,
            at_token,
            derive_token,
            lpar_token,
            trais,
            rpar_token,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct DeriveDecrSynDecl {
    #[id]
    pub path: DecrPath,
    #[return_ref]
    pub trais: SmallVec<[TraitObelisk; 8]>,
    pub syn_expr_region: SynExprRegion,
}

impl DeriveDecrSynDecl {
    #[inline(always)]
    pub(super) fn from_node_decl(
        db: &dyn SynDeclDb,
        path: DecrPath,
        syn_node_decl: DeriveDecrSynNodeDecl,
    ) -> DeclResult<Self> {
        let trais = SmallVec::from(syn_node_decl.trais(db).as_ref()?.elements());
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(Self::new(db, path, trais, syn_expr_region))
    }
}
