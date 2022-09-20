use crate::*;
use husky_entity_semantics::{EntityRouteStore, StoreEntityRoute};
use husky_linkage_table::{LinkageTable, ResolveLinkage};
use husky_static_defn::ResolveStaticRootDefn;
use infer_total::InferQueryGroup;
use upcast::Upcast;

impl fmt::Debug for Comptime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("husky-compilerompileTime").finish()
    }
}

impl salsa::Database for Comptime {}

impl salsa::ParallelDatabase for Comptime {
    fn snapshot(&self) -> salsa::Snapshot<Comptime> {
        salsa::Snapshot::new(Comptime {
            storage: self.storage.snapshot(),
            file_interner: self.file_interner.clone(),
            word_interner: self.word_interner.clone(),
            entity_route_interner: self.entity_route_interner.clone(),
            live_docs: self.live_docs.clone(),
            linkage_table: self.linkage_table.clone(),
            entity_route_store: self.entity_route_store.clone(),
            config: self.config.clone(),
        })
    }
}

impl AllocateUniqueFile for Comptime {
    fn file_interner(&self) -> &husky_file::FileInterner {
        &self.file_interner
    }
}

impl InternWord for Comptime {
    fn word_allocator(&self) -> &husky_word::WordInterner {
        &self.word_interner
    }
}

impl LiveFiles for Comptime {
    fn get_live_files(&self) -> &ASafeRwLock<IndexMap<husky_file::FilePtr, ASafeRwLock<String>>> {
        &self.live_docs
    }

    fn did_change_source(&mut self, id: husky_file::FilePtr) {
        husky_file::FileContentQuery.in_db_mut(self).invalidate(&id);
    }
}

impl FileQueryGroup for Comptime {}

impl InternEntityRoute for Comptime {
    fn entity_route_interner(&self) -> &husky_entity_route::EntityRouteInterner {
        &self.entity_route_interner
    }
}

impl TokenQueryGroup for Comptime {}

impl ResolveStaticRootDefn for Comptime {
    fn __root_defn_resolver(
        &self,
    ) -> fn(ident: husky_word::RootIdentifier) -> &'static husky_static_defn::EntityStaticDefn {
        self.config.__resolve_root_defn
    }
}

impl EntitySyntaxQueryGroup for Comptime {}

impl AstQueryGroup for Comptime {}

impl Upcast<dyn InferQueryGroup> for Comptime {
    fn upcast(&self) -> &(dyn infer_total::InferQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn husky_entity_semantics::EntityDefnQueryGroup> for Comptime {
    fn upcast(&self) -> &(dyn husky_entity_semantics::EntityDefnQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn husky_entity_syntax::EntitySyntaxSalsaQueryGroup> for Comptime {
    fn upcast(&self) -> &(dyn husky_entity_syntax::EntitySyntaxSalsaQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn husky_entity_syntax::EntitySyntaxQueryGroup> for Comptime {
    fn upcast(&self) -> &(dyn husky_entity_syntax::EntitySyntaxQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn DeclQueryGroup> for Comptime {
    fn upcast(&self) -> &(dyn DeclQueryGroup + 'static) {
        self
    }
}

impl infer_contract::InferContractQueryGroup for Comptime {}

impl infer_total::InferQueryGroup for Comptime {}

impl ResolveLinkage for Comptime {
    fn linkage_table(&self) -> &LinkageTable {
        &self.linkage_table
    }
}

impl StoreEntityRoute for Comptime {
    fn entity_route_store(&self) -> &EntityRouteStore {
        &self.entity_route_store
    }
}
