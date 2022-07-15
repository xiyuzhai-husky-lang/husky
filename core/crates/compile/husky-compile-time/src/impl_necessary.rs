use std::{collections::HashMap, sync::Mutex};

use crate::*;
use husky_entity_semantics::{EntityRouteStore, StoreEntityRoute};
use husky_linkage_table::{LinkageTable, ResolveLinkage};
use husky_trace_protocol::*;
use infer_total::InferQueryGroup;
use static_defn::ResolveStaticRootDefn;
use upcast::Upcast;
use vm::{InterpreterQueryGroup, __AnyValueDyn};

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
            file_interner: self.file_interner.clone(),
            word_interner: self.word_interner.clone(),
            entity_route_interner: self.entity_route_interner.clone(),
            live_docs: self.live_docs.clone(),
            linkage_table: self.linkage_table.clone(),
            entity_route_store: self.entity_route_store.clone(),
            opt_main: self.opt_main,
            config: self.config.clone(),
            ty_cache: self.ty_cache.clone(),
            entity_route_menu: self.entity_route_menu.clone(),
        })
    }
}

impl AllocateUniqueFile for HuskyCompileTime {
    fn file_interner(&self) -> &husky_file::FileInterner {
        &self.file_interner
    }
}

impl InternWord for HuskyCompileTime {
    fn word_allocator(&self) -> &word::WordInterner {
        &self.word_interner
    }
}

impl LiveFiles for HuskyCompileTime {
    fn get_live_files(&self) -> &ASafeRwLock<IndexMap<husky_file::FilePtr, ASafeRwLock<String>>> {
        &self.live_docs
    }

    fn did_change_source(&mut self, id: husky_file::FilePtr) {
        husky_file::FileContentQuery.in_db_mut(self).invalidate(&id);
    }
}

impl FileQueryGroup for HuskyCompileTime {}

impl InternEntityRoute for HuskyCompileTime {
    fn scope_interner(&self) -> &husky_entity_route::EntityRouteInterner {
        &self.entity_route_interner
    }
}

impl TokenQueryGroup for HuskyCompileTime {}

impl ResolveStaticRootDefn for HuskyCompileTime {
    fn __root_defn_resolver(
        &self,
    ) -> fn(ident: word::RootIdentifier) -> &'static static_defn::EntityStaticDefn {
        self.config.__root_defn_resolver
    }
}

impl EntitySyntaxQueryGroup for HuskyCompileTime {
    fn opt_package_main(&self) -> Option<FilePtr> {
        self.opt_main
    }
}

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
    fn husky_linkage_table(&self) -> &LinkageTable {
        &self.linkage_table
    }
}

impl StoreEntityRoute for HuskyCompileTime {
    fn entity_route_store(&self) -> &EntityRouteStore {
        &self.entity_route_store
    }
}
