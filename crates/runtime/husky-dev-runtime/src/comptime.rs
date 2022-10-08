use crate::*;
use husky_diagnostics::HuskyDiagnosticQuery;
use husky_entity_semantics::{EntityRouteStore, StoreEntityRoute};
use husky_linkage_table::LinkageTable;
use husky_static_defn::ResolveStaticRootDefn;
use upcast::Upcast;

impl AllocateUniqueFile for HuskyDevRuntime {
    fn file_interner(&self) -> &husky_file::FileInterner {
        &self.file_interner
    }
}

impl InternWord for HuskyDevRuntime {
    fn word_allocator(&self) -> &husky_word::WordInterner {
        &self.word_interner
    }
}

impl LiveFiles for HuskyDevRuntime {
    fn get_live_files(&self) -> &ASafeRwLock<IndexMap<husky_file::FilePtr, ASafeRwLock<String>>> {
        &self.live_docs
    }

    fn did_change_source(&mut self, id: husky_file::FilePtr) {
        husky_file::FileContentQuery.in_db_mut(self).invalidate(&id);
    }
}

impl FileQueryGroup for HuskyDevRuntime {}

impl InternEntityRoute for HuskyDevRuntime {
    fn entity_route_interner(&self) -> &husky_entity_route::EntityRouteInterner {
        &self.entity_route_interner
    }
}

impl TokenQueryGroup for HuskyDevRuntime {}

impl ResolveStaticRootDefn for HuskyDevRuntime {
    fn __root_defn_resolver(
        &self,
    ) -> fn(ident: husky_word::RootBuiltinIdentifier) -> &'static husky_static_defn::EntityStaticDefn
    {
        self.config.comptime.__resolve_root_defn
    }
}

impl EntitySyntaxQueryGroup for HuskyDevRuntime {}

impl AstQueryGroup for HuskyDevRuntime {}

impl Upcast<dyn InferQueryGroup> for HuskyDevRuntime {
    fn upcast(&self) -> &(dyn infer_total::InferQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn husky_entity_semantics::EntityDefnQueryGroup> for HuskyDevRuntime {
    fn upcast(&self) -> &(dyn husky_entity_semantics::EntityDefnQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn husky_entity_syntax::EntitySyntaxSalsaQueryGroup> for HuskyDevRuntime {
    fn upcast(&self) -> &(dyn husky_entity_syntax::EntitySyntaxSalsaQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn husky_entity_syntax::EntitySyntaxQueryGroup> for HuskyDevRuntime {
    fn upcast(&self) -> &(dyn husky_entity_syntax::EntitySyntaxQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn DeclQueryGroup> for HuskyDevRuntime {
    fn upcast(&self) -> &(dyn DeclQueryGroup + 'static) {
        self
    }
}

impl infer_contract::InferContractQueryGroup for HuskyDevRuntime {}

impl infer_total::InferQueryGroup for HuskyDevRuntime {}

impl ResolveLinkage for HuskyDevRuntime {
    fn linkage_table(&self) -> &LinkageTable {
        &self.linkage_table
    }
}

impl StoreEntityRoute for HuskyDevRuntime {
    fn entity_route_store(&self) -> &EntityRouteStore {
        &self.entity_route_store
    }
}

impl HuskyDiagnosticQuery for HuskyDevRuntime {}

impl ComptimeOps for HuskyDevRuntime {
    fn comptime_config(&self) -> &ComptimeConfig {
        &self.config.comptime
    }
}

impl ComptimeQueryGroup for HuskyDevRuntime {}
