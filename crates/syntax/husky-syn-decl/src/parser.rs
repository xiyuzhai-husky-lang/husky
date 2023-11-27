use crate::*;

use husky_decl_ast::DeclAst;

use husky_entity_syn_tree::{helpers::tokra_region::DeclTokraRegionData, *};
use husky_print_utils::p;

pub(crate) struct DeclParser<'a> {
    db: &'a ::salsa::Db,
    syn_node_path: ItemSynNodePath,
    module_symbol_context: ModuleSymbolContext<'a>,
    tokra_region_data: DeclTokraRegionData<'a>,
}

impl<'a> DeclParser<'a> {
    pub(crate) fn new(db: &'a ::salsa::Db, syn_node_path: ItemSynNodePath) -> Self {
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
    pub(crate) fn expr_parser(
        &self,
        parent_expr_region: Option<SynExprRegion>,
        allow_self_type: AllowSelfType,
        allow_self_value: AllowSelfValue,
        env: Option<ExprEnvironment>,
    ) -> SynDeclExprParser<'a> {
        SynExprContext::new(
            self.db,
            SynNodeRegionPath::Decl(self.syn_node_path.into()),
            self.module_symbol_context,
            parent_expr_region,
            allow_self_type,
            allow_self_value,
        )
        .expr_parser(env, self.tokra_region_data.regional_token_stream())
    }

    #[inline(always)]
    pub(crate) fn db(&self) -> &'a ::salsa::Db {
        self.db
    }

    #[inline(always)]
    pub(crate) fn ast(&self) -> &'a DeclAst {
        todo!()
    }

    pub(crate) fn syn_node_path(&self) -> ItemSynNodePath {
        self.syn_node_path
    }
}
