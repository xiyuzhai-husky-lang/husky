use crate::*;
use husky_diagnostics::HuskyDiagnosticQuery;
use husky_entity_semantics::{EntityRouteStore, StoreEntityRoute};
use husky_linkage_table::LinkageTable;
use husky_static_defn::ResolveStaticRootDefn;
use upcast::Upcast;

impl InternPath for DevRuntime {
    fn path_itr(&self) -> &husky_path::PathInterner {
        todo!()
    }
}

impl InternHuskyPath for DevRuntime {}

impl InternWord for DevRuntime {
    fn word_itr(&self) -> &husky_word::WordInterner {
        &self.word_interner
    }
}

impl VfsQueryGroupBase for DevRuntime {
    fn get_live_files(&self) -> &ASafeRwLock<IndexMap<husky_path::PathItd, ASafeRwLock<String>>> {
        &self.live_docs
    }

    fn did_change_source(&mut self, id: husky_path::PathItd) {
        husky_path::FileContentQuery.in_db_mut(self).invalidate(&id);
    }
}

impl FileQueryGroup for DevRuntime {}

impl ResolveStaticRootDefn for DevRuntime {
    fn __root_defn_resolver(
        &self,
    ) -> fn(ident: husky_word::RootBuiltinIdentifier) -> &'static husky_static_defn::EntityStaticDefn
    {
        self.config.comptime.__resolve_root_defn
    }
}

impl EntitySyntaxQueryGroup for DevRuntime {}

impl AstQueryGroup for DevRuntime {}

impl Upcast<dyn husky_entity_semantics::EntityDefnQueryGroup> for DevRuntime {
    fn upcast(&self) -> &(dyn husky_entity_semantics::EntityDefnQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn husky_entity_syntax::EntitySyntaxSalsaQueryGroup> for DevRuntime {
    fn upcast(&self) -> &(dyn husky_entity_syntax::EntitySyntaxSalsaQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn husky_entity_syntax::EntitySyntaxQueryGroup> for DevRuntime {
    fn upcast(&self) -> &(dyn husky_entity_syntax::EntitySyntaxQueryGroup + 'static) {
        self
    }
}

impl ResolveLinkage for DevRuntime {
    fn linkage_table(&self) -> &LinkageTable {
        &self.linkage_table
    }
}

impl StoreEntityRoute for DevRuntime {
    fn entity_route_store(&self) -> &EntityRouteStore {
        &self.entity_route_store
    }
}

impl HuskyDiagnosticQuery for DevRuntime {}

impl ComptimeOps for DevRuntime {
    fn comptime_config(&self) -> &ComptimeConfig {
        &self.config.comptime
    }
}

impl ComptimeQueryGroup for DevRuntime {}
