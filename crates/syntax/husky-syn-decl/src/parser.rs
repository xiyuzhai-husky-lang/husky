use crate::*;
use husky_ast::*;

use husky_coword::Ident;
use husky_entity_syn_tree::{
    tokra_region::{DeclTokraRegionData, HasDeclTokraRegion},
    *,
};
use husky_entity_taxonomy::*;
use husky_print_utils::p;
use husky_token::*;
use parsec::*;

pub(crate) struct DeclParserFactory<'a> {
    db: &'a dyn SynDeclDb,
    module_symbol_context: ModuleSymbolContext<'a>,
    token_sheet_data: &'a TokenSheetData,
    ast_sheet: &'a AstSheet,
    tokra_region_data: DeclTokraRegionData<'a>,
}

impl<'a> DeclParserFactory<'a> {
    pub(crate) fn new<P>(db: &'a dyn SynDeclDb, syn_node_path: P) -> Self
    where
        P: HasDeclTokraRegion,
    {
        let path = syn_node_path.module_path(db);
        let Ok(module_symbol_context) = db.module_symbol_context(path) else {
            use salsa::DebugWithDb;
            p!(path.debug(db));
            unreachable!("valid module")
        };
        Self {
            db,
            module_symbol_context,
            token_sheet_data: db.token_sheet_data(path).expect("valid module"),
            ast_sheet: db.ast_sheet(path).expect("valid module"),
            tokra_region_data: syn_node_path.decl_tokra_region(db).data(db),
        }
    }

    #[inline(always)]
    pub(crate) fn parser(
        &self,
        syn_node_path: impl Into<ItemSynNodePath>,
        parent_expr_region: Option<SynExprRegion>,
        allow_self_type: AllowSelfType,
        allow_self_value: AllowSelfValue,
        env: Option<ExprEnvironment>,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: impl Into<Option<TokenStreamState>>,
    ) -> SynDeclExprParser<'a> {
        let token_stream = self
            .token_sheet_data
            .token_group_token_stream(token_group_idx, saved_stream_state);
        SynExprContext::new(
            self.db,
            RegionPath::Decl(syn_node_path.into()),
            self.module_symbol_context,
            parent_expr_region,
            allow_self_type,
            allow_self_value,
        )
        .expr_parser(env, token_stream)
    }

    #[inline(always)]
    pub(crate) fn db(&self) -> &'a dyn SynDeclDb {
        self.db
    }

    #[inline(always)]
    pub(crate) fn ast_sheet(&self) -> &'a AstSheet {
        self.ast_sheet
    }

    pub(crate) fn token_sheet_data(&self) -> &'a TokenSheetData {
        self.token_sheet_data
    }
}
