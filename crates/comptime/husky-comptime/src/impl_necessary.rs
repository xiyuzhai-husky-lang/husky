use std::{collections::HashMap, sync::Mutex};

use crate::*;
use husky_entity_semantics::{EntityRouteStore, StoreEntityRoute};
use husky_linkage_table::{LinkageTable, ResolveLinkage};
use husky_static_defn::ResolveStaticRootDefn;
use husky_trace_protocol::*;
use husky_vm::InterpreterQueryGroup;
use infer_total::InferQueryGroup;
use upcast::Upcast;

impl fmt::Debug for HuskyComptime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("husky-compilerompileTime").finish()
    }
}

impl salsa::Database for HuskyComptime {}

impl salsa::ParallelDatabase for HuskyComptime {
    fn snapshot(&self) -> salsa::Snapshot<HuskyComptime> {
        salsa::Snapshot::new(HuskyComptime {
            storage: self.storage.snapshot(),
            file_interner: self.file_interner.clone(),
            word_interner: self.word_interner.clone(),
            entity_route_interner: self.entity_route_interner.clone(),
            live_docs: self.live_docs.clone(),
            linkage_table: self.linkage_table.clone(),
            entity_route_store: self.entity_route_store.clone(),
            config: self.config.clone(),
            ty_cache: self.ty_cache.clone(),
        })
    }
}

impl AllocateUniqueFile for HuskyComptime {
    fn file_interner(&self) -> &husky_file::FileInterner {
        &self.file_interner
    }
}

impl InternWord for HuskyComptime {
    fn word_allocator(&self) -> &husky_word::WordInterner {
        &self.word_interner
    }
}

impl LiveFiles for HuskyComptime {
    fn get_live_files(&self) -> &ASafeRwLock<IndexMap<husky_file::FilePtr, ASafeRwLock<String>>> {
        &self.live_docs
    }

    fn did_change_source(&mut self, id: husky_file::FilePtr) {
        husky_file::FileContentQuery.in_db_mut(self).invalidate(&id);
    }
}

impl FileQueryGroup for HuskyComptime {}

impl InternEntityRoute for HuskyComptime {
    fn entity_route_interner(&self) -> &husky_entity_route::EntityRouteInterner {
        &self.entity_route_interner
    }
}

impl TokenQueryGroup for HuskyComptime {}

impl ResolveStaticRootDefn for HuskyComptime {
    fn __root_defn_resolver(
        &self,
    ) -> fn(ident: husky_word::RootIdentifier) -> &'static husky_static_defn::EntityStaticDefn {
        self.config.__resolve_root_defn
    }
}

impl EntitySyntaxQueryGroup for HuskyComptime {}

impl AstQueryGroup for HuskyComptime {}

impl Upcast<dyn InferQueryGroup> for HuskyComptime {
    fn upcast(&self) -> &(dyn infer_total::InferQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn husky_entity_semantics::EntityDefnQueryGroup> for HuskyComptime {
    fn upcast(&self) -> &(dyn husky_entity_semantics::EntityDefnQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn husky_entity_syntax::EntitySyntaxSalsaQueryGroup> for HuskyComptime {
    fn upcast(&self) -> &(dyn husky_entity_syntax::EntitySyntaxSalsaQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn husky_entity_syntax::EntitySyntaxQueryGroup> for HuskyComptime {
    fn upcast(&self) -> &(dyn husky_entity_syntax::EntitySyntaxQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn DeclQueryGroup> for HuskyComptime {
    fn upcast(&self) -> &(dyn DeclQueryGroup + 'static) {
        self
    }
}

impl infer_contract::InferContractQueryGroup for HuskyComptime {}

impl infer_total::InferQueryGroup for HuskyComptime {}

impl ResolveLinkage for HuskyComptime {
    fn linkage_table(&self) -> &LinkageTable {
        &self.linkage_table
    }
}

impl StoreEntityRoute for HuskyComptime {
    fn entity_route_store(&self) -> &EntityRouteStore {
        &self.entity_route_store
    }
}
