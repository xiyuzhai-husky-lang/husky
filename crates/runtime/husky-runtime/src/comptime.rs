use crate::*;
use husky_diagnostics::DiagnosticQuery;
use husky_entity_semantics::{EntityRouteStore, StoreEntityRoute};
use husky_linkage_table::LinkageTable;
use husky_static_defn::ResolveStaticRootDefn;
use upcast::Upcast;

impl AllocateUniqueFile for Runtime {
    fn file_interner(&self) -> &husky_file::FileInterner {
        &self.file_interner
    }
}

impl InternWord for Runtime {
    fn word_allocator(&self) -> &husky_word::WordInterner {
        &self.word_interner
    }
}

impl LiveFiles for Runtime {
    fn get_live_files(&self) -> &ASafeRwLock<IndexMap<husky_file::FilePtr, ASafeRwLock<String>>> {
        &self.live_docs
    }

    fn did_change_source(&mut self, id: husky_file::FilePtr) {
        husky_file::FileContentQuery.in_db_mut(self).invalidate(&id);
    }
}

impl FileQueryGroup for Runtime {}

impl InternEntityRoute for Runtime {
    fn entity_route_interner(&self) -> &husky_entity_route::EntityRouteInterner {
        &self.entity_route_interner
    }
}

impl TokenQueryGroup for Runtime {}

impl ResolveStaticRootDefn for Runtime {
    fn __root_defn_resolver(
        &self,
    ) -> fn(ident: husky_word::RootIdentifier) -> &'static husky_static_defn::EntityStaticDefn {
        self.config.comptime.__resolve_root_defn
    }
}

impl EntitySyntaxQueryGroup for Runtime {}

impl AstQueryGroup for Runtime {}

impl Upcast<dyn InferQueryGroup> for Runtime {
    fn upcast(&self) -> &(dyn infer_total::InferQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn husky_entity_semantics::EntityDefnQueryGroup> for Runtime {
    fn upcast(&self) -> &(dyn husky_entity_semantics::EntityDefnQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn husky_entity_syntax::EntitySyntaxSalsaQueryGroup> for Runtime {
    fn upcast(&self) -> &(dyn husky_entity_syntax::EntitySyntaxSalsaQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn husky_entity_syntax::EntitySyntaxQueryGroup> for Runtime {
    fn upcast(&self) -> &(dyn husky_entity_syntax::EntitySyntaxQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn DeclQueryGroup> for Runtime {
    fn upcast(&self) -> &(dyn DeclQueryGroup + 'static) {
        self
    }
}

impl infer_contract::InferContractQueryGroup for Runtime {}

impl infer_total::InferQueryGroup for Runtime {}

impl ResolveLinkage for Runtime {
    fn linkage_table(&self) -> &LinkageTable {
        &self.linkage_table
    }
}

impl StoreEntityRoute for Runtime {
    fn entity_route_store(&self) -> &EntityRouteStore {
        &self.entity_route_store
    }
}

impl DiagnosticQuery for Runtime {}
