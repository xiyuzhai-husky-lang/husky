use crate::*;

use husky_coword::Ident;
use husky_decl_ast::DeclAst;
use husky_entity_syn_tree::{
    helpers::tokra_region::{DeclTokraRegionData, HasDeclTokraRegion},
    *,
};
use husky_entity_taxonomy::*;
use husky_print_utils::p;
use husky_regional_token::*;
use parsec::*;

pub(crate) struct DeclParserFactory<'a, P>
where
    P: HasDeclTokraRegion,
{
    db: &'a dyn SynDeclDb,
    syn_node_path: P,
    module_symbol_context: ModuleSymbolContext<'a>,
    tokra_region_data: DeclTokraRegionData<'a>,
}

impl<'a, P> DeclParserFactory<'a, P>
where
    P: HasDeclTokraRegion,
{
    pub(crate) fn new(db: &'a dyn SynDeclDb, syn_node_path: P) -> Self {
        let path = syn_node_path.module_path(db);
        let Ok(module_symbol_context) = db.module_symbol_context(path) else {
            use salsa::DebugWithDb;
            p!(path.debug(db));
            unreachable!("valid module")
        };
        Self {
            db,
            syn_node_path,
            module_symbol_context,
            tokra_region_data: syn_node_path.decl_tokra_region(db).data(db),
        }
    }

    #[inline(always)]
    pub(crate) fn parser(
        &self,
        parent_expr_region: Option<SynExprRegion>,
        allow_self_type: AllowSelfType,
        allow_self_value: AllowSelfValue,
        env: Option<ExprEnvironment>,
    ) -> SynDeclExprParser<'a> {
        todo!()
        // let token_stream = self
        //     .token_sheet_data
        //     .token_group_token_stream(token_group_idx, saved_stream_state);
        // SynExprContext::new(
        //     self.db,
        //     RegionPath::Decl(syn_node_path.into()),
        //     self.module_symbol_context,
        //     parent_expr_region,
        //     allow_self_type,
        //     allow_self_value,
        // )
        // .expr_parser(env, token_stream)
    }

    #[inline(always)]
    pub(crate) fn db(&self) -> &'a dyn SynDeclDb {
        self.db
    }

    #[inline(always)]
    pub(crate) fn ast(&self) -> &'a DeclAst {
        todo!()
    }

    pub(crate) fn syn_node_path(&self) -> P {
        self.syn_node_path
    }
}
