use crate::*;
use husky_entity_semantics::{EntityRouteStore, StoreEntityRoute};
use husky_linkage_table::{LinkageSourceTable, ResolveLinkage};
use husky_trace_protocol::*;
use infer_total::InferQueryGroup;
use static_defn::ResolveStaticRootDefn;
use upcast::Upcast;
use vm::{AnyValueDyn, InterpreterQueryGroup};

impl fmt::Debug for HuskyCompileTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HuskyCompileTime").finish()
    }
}

impl salsa::Database for HuskyCompileTime {}

impl salsa::ParallelDatabase for HuskyCompileTime {
    fn snapshot(&self) -> salsa::Snapshot<HuskyCompileTime> {
        salsa::Snapshot::new(HuskyCompileTime {
            storage: self.storage.snapshot(),
            file_unique_allocator: self.file_unique_allocator.clone(),
            word_unique_allocator: self.word_unique_allocator.clone(),
            scope_unique_allocator: self.scope_unique_allocator.clone(),
            live_docs: self.live_docs.clone(),
            husky_linkage_table: self.husky_linkage_table.clone(),
            entity_route_store: self.entity_route_store.clone(),
            opt_main: self.opt_main,
            __root_defn_resolver: self.__root_defn_resolver,
        })
    }
}

impl AllocateUniqueFile for HuskyCompileTime {
    fn file_interner(&self) -> &husky_file::FileInterner {
        &self.file_unique_allocator
    }
}

impl InternWord for HuskyCompileTime {
    fn word_allocator(&self) -> &word::WordAllocator {
        &self.word_unique_allocator
    }
}

impl LiveFiles for HuskyCompileTime {
    fn get_live_files(&self) -> &ARwLock<IndexMap<husky_file::FilePtr, ARwLock<String>>> {
        &self.live_docs
    }

    fn did_change_source(&mut self, id: husky_file::FilePtr) {
        husky_file::FileContentQuery.in_db_mut(self).invalidate(&id);
    }
}

impl FileQueryGroup for HuskyCompileTime {}

impl AllocateUniqueScope for HuskyCompileTime {
    fn scope_unique_allocator(&self) -> &husky_entity_route_syntax::EntityRouteInterner {
        &self.scope_unique_allocator
    }
}

impl TokenQueryGroup for HuskyCompileTime {}

impl ResolveStaticRootDefn for HuskyCompileTime {
    fn __root_defn_resolver(
        &self,
    ) -> fn(ident: word::RootIdentifier) -> &'static static_defn::EntityStaticDefn {
        self.__root_defn_resolver
    }
}

impl EntitySyntaxQueryGroup for HuskyCompileTime {}

impl AstQueryGroup for HuskyCompileTime {}

impl Upcast<dyn InferQueryGroup> for HuskyCompileTime {
    fn upcast(&self) -> &(dyn infer_total::InferQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn husky_entity_semantics::EntityDefnQueryGroup> for HuskyCompileTime {
    fn upcast(&self) -> &(dyn husky_entity_semantics::EntityDefnQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn husky_entity_syntax::EntitySyntaxSalsaQueryGroup> for HuskyCompileTime {
    fn upcast(&self) -> &(dyn husky_entity_syntax::EntitySyntaxSalsaQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn husky_entity_syntax::EntitySyntaxQueryGroup> for HuskyCompileTime {
    fn upcast(&self) -> &(dyn husky_entity_syntax::EntitySyntaxQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn DeclQueryGroup> for HuskyCompileTime {
    fn upcast(&self) -> &(dyn DeclQueryGroup + 'static) {
        self
    }
}

impl infer_contract::InferContractQueryGroup for HuskyCompileTime {}

impl infer_total::InferQueryGroup for HuskyCompileTime {}

impl ResolveLinkage for HuskyCompileTime {
    fn husky_linkage_table(&self) -> &LinkageSourceTable {
        &self.husky_linkage_table
    }
}

impl StoreEntityRoute for HuskyCompileTime {
    fn entity_route_store(&self) -> &EntityRouteStore {
        &self.entity_route_store
    }
}
