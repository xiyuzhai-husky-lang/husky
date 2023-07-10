use crate::*;
use husky_diagnostics::DiagnosticsDb;
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

impl VfsQueryGroupBase for DevRuntime {
    fn get_live_files(&self) -> Option<&ASafeRwLock<IndexMap<EntityPath, ASafeRwLock<String>>>> {
        Some(&self.live_docs)
    }

    fn watch(&self, path: DiffPath) {
        todo!()
    }
}

impl FileQueryGroup for DevRuntime {}

impl ResolveStaticRootDefn for DevRuntime {
    fn __root_defn_resolver(
        &self,
    ) -> fn(ident: husky_coword::RootBuiltinIdent) -> &'static husky_static_defn::EntityStaticDefn
    {
        self.config.comptime.__resolve_root_defn
    }
}

impl EntityTreeDb for DevRuntime {}

impl AstDb for DevRuntime {}

impl Upcast<dyn husky_entity_semantics::EntityDefnQueryGroup> for DevRuntime {
    fn upcast(&self) -> &(dyn husky_entity_semantics::EntityDefnQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn husky_entity_tree::EntityTreeDb> for DevRuntime {
    fn upcast(&self) -> &(dyn husky_entity_tree::EntityTreeDb + 'static) {
        self
    }
}

impl Upcast<dyn husky_entity_tree::EntityTreeDb> for DevRuntime {
    fn upcast(&self) -> &(dyn husky_entity_tree::EntityTreeDb + 'static) {
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

impl DiagnosticsDb for DevRuntime {}

impl ComptimeOps for DevRuntime {
    fn comptime_config(&self) -> &ComptimeConfig {
        &self.config.comptime
    }
}

impl ComptimeQueryGroup for DevRuntime {}
