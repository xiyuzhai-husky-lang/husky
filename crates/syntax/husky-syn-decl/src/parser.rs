use crate::*;
use husky_entity_tree::{
    helpers::tokra_region::crate_decl::{CrateDeclTokraRegionDataRef, HasCrateDeclTokraRegion},
    jar::EntityTreeDb,
};
use husky_entity_tree::{
    helpers::tokra_region::ItemDeclTokraRegionDataRef, node::ItemSynNodePath,
    region_path::SynNodeRegionPath, symbol::ModuleSymbolContext,
};
use husky_vfs::CratePath;

pub(crate) struct ItemDeclParser<'db> {
    db: &'db ::salsa::Db,
    syn_node_path: ItemSynNodePath,
    module_symbol_context: ModuleSymbolContext<'db>,
    tokra_region_data: ItemDeclTokraRegionDataRef<'db>,
}

impl<'db> ItemDeclParser<'db> {
    pub(crate) fn new(db: &'db ::salsa::Db, syn_node_path: ItemSynNodePath) -> Self {
        let module_path = syn_node_path.module_path(db);
        let Ok(module_symbol_context) = db.module_symbol_context(module_path) else {
            unreachable!("expected valid module")
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
    ) -> SynDeclExprParser<'db> {
        SynExprContext::new2(
            self.db,
            SynNodeRegionPath::ItemDecl(self.syn_node_path.into()),
            self.module_symbol_context,
            parent_expr_region,
            allow_self_type,
            allow_self_value,
        )
        .unwrap()
        .token_stream_expr_parser(env, self.tokra_region_data.regional_token_stream())
    }

    #[inline(always)]
    pub(crate) fn db(&self) -> &'db ::salsa::Db {
        self.db
    }

    pub(crate) fn syn_node_path(&self) -> ItemSynNodePath {
        self.syn_node_path
    }
}

pub(crate) struct CrateDeclParser<'db> {
    db: &'db ::salsa::Db,
    crate_path: CratePath,
    module_symbol_context: ModuleSymbolContext<'db>,
    tokra_region_data: CrateDeclTokraRegionDataRef<'db>,
}

/// # constructor

impl<'db> CrateDeclParser<'db> {
    pub(crate) fn new(db: &'db ::salsa::Db, crate_path: CratePath) -> Self {
        let module_path = crate_path.root_module_path(db);
        let Ok(module_symbol_context) = db.module_symbol_context(module_path) else {
            unreachable!("expected valid module")
        };
        Self {
            db,
            crate_path,
            module_symbol_context,
            tokra_region_data: crate_path.decl_tokra_region(db).data(db),
        }
    }
}

/// # getters

impl<'db> CrateDeclParser<'db> {
    pub(crate) fn db(&self) -> &'db ::salsa::Db {
        self.db
    }

    pub(crate) fn crate_path(&self) -> CratePath {
        self.crate_path
    }
}

/// # factory
impl<'db> CrateDeclParser<'db> {
    #[inline(always)]
    pub(crate) fn expr_parser(
        &self,
        parent_expr_region: Option<SynExprRegion>,
        allow_self_type: AllowSelfType,
        allow_self_value: AllowSelfValue,
        env: Option<ExprEnvironment>,
    ) -> SynDeclExprParser<'db> {
        SynExprContext::new2(
            self.db,
            SynNodeRegionPath::CrateDecl(self.crate_path),
            self.module_symbol_context,
            parent_expr_region,
            allow_self_type,
            allow_self_value,
        )
        .unwrap()
        .token_stream_expr_parser(env, self.tokra_region_data.regional_token_stream())
    }
}
