use crate::*;
use husky_diagnostics::DiagnosticQuery;
use husky_entity_semantics::{EntityRouteStore, StoreEntityRoute};
use husky_linkage_table::LinkageTable;
use husky_static_defn::ResolveStaticRootDefn;
use upcast::Upcast;

impl AllocateUniqueFile for HuskyRuntime {
    fn file_interner(&self) -> &husky_file::FileInterner {
        &self.file_interner
    }
}

impl InternWord for HuskyRuntime {
    fn word_allocator(&self) -> &husky_word::WordInterner {
        &self.word_interner
    }
}

impl LiveFiles for HuskyRuntime {
    fn get_live_files(&self) -> &ASafeRwLock<IndexMap<husky_file::FilePtr, ASafeRwLock<String>>> {
        &self.live_docs
    }

    fn did_change_source(&mut self, id: husky_file::FilePtr) {
        husky_file::FileContentQuery.in_db_mut(self).invalidate(&id);
    }
}

impl FileQueryGroup for HuskyRuntime {}

impl InternEntityRoute for HuskyRuntime {
    fn entity_route_interner(&self) -> &husky_entity_route::EntityRouteInterner {
        &self.entity_route_interner
    }
}

impl TokenQueryGroup for HuskyRuntime {}

impl ResolveStaticRootDefn for HuskyRuntime {
    fn __root_defn_resolver(
        &self,
    ) -> fn(ident: husky_word::RootIdentifier) -> &'static husky_static_defn::EntityStaticDefn {
        self.config.comptime.__resolve_root_defn
    }
}

impl EntitySyntaxQueryGroup for HuskyRuntime {}

impl AstQueryGroup for HuskyRuntime {}

impl Upcast<dyn InferQueryGroup> for HuskyRuntime {
    fn upcast(&self) -> &(dyn infer_total::InferQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn husky_entity_semantics::EntityDefnQueryGroup> for HuskyRuntime {
    fn upcast(&self) -> &(dyn husky_entity_semantics::EntityDefnQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn husky_entity_syntax::EntitySyntaxSalsaQueryGroup> for HuskyRuntime {
    fn upcast(&self) -> &(dyn husky_entity_syntax::EntitySyntaxSalsaQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn husky_entity_syntax::EntitySyntaxQueryGroup> for HuskyRuntime {
    fn upcast(&self) -> &(dyn husky_entity_syntax::EntitySyntaxQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn DeclQueryGroup> for HuskyRuntime {
    fn upcast(&self) -> &(dyn DeclQueryGroup + 'static) {
        self
    }
}

impl infer_contract::InferContractQueryGroup for HuskyRuntime {}

impl infer_total::InferQueryGroup for HuskyRuntime {}

impl ResolveLinkage for HuskyRuntime {
    fn linkage_table(&self) -> &LinkageTable {
        &self.linkage_table
    }
}

impl StoreEntityRoute for HuskyRuntime {
    fn entity_route_store(&self) -> &EntityRouteStore {
        &self.entity_route_store
    }
}

impl DiagnosticQuery for HuskyRuntime {}
