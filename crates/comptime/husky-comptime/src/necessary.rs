use crate::*;
use husky_entity_semantics::{EntityRouteStore, StoreEntityRoute};
use husky_linkage_table::{LinkageTable, ResolveLinkage};
use husky_path::LiveFileSupport;
use husky_static_defn::ResolveStaticRootDefn;
use interner::path::InternPath;
use upcast::{Upcast, UpcastMut};

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
            // entity_route_interner: self.entity_route_interner.clone(),
            live_docs: self.live_docs.clone(),
            linkage_table: self.linkage_table.clone(),
            entity_route_store: self.entity_route_store.clone(),
            config: self.config.clone(),
        })
    }
}

impl InternPath for HuskyComptime {
    fn path_itr(&self) -> &interner::path::PathInterner {
        todo!()
    }
}

impl InternHuskyPath for HuskyComptime {}

impl VfsQueryGroupBase for HuskyComptime {
    fn get_live_files(
        &self,
    ) -> Option<&ASafeRwLock<IndexMap<husky_path::PathItd, ASafeRwLock<String>>>> {
        Some(&self.live_docs)
    }

    fn watch(&self, path: PathItd) {
        todo!()
    }
}

impl FileQueryGroup for HuskyComptime {}

impl UpcastMut<dyn FileSalsaQuery> for HuskyComptime {
    fn upcast_mut(&mut self) -> &mut (dyn FileSalsaQuery + 'static) {
        self
    }
}

impl LiveFileSupport for HuskyComptime {}

// impl InternEntityRoute for HuskyComptime {
//     fn entity_route_interner(&self) -> &husky_term::EntityRouteInterner {
//         &self.entity_route_interner
//     }
// }

impl ResolveStaticRootDefn for HuskyComptime {
    fn __root_defn_resolver(
        &self,
    ) -> fn(
        ident: husky_identifier::RootBuiltinIdentifier,
    ) -> &'static husky_static_defn::EntityStaticDefn {
        self.config.__resolve_root_defn
    }
}

impl EntityTreeDb for HuskyComptime {}

impl AstDb for HuskyComptime {}

impl Upcast<dyn husky_entity_semantics::EntityDefnQueryGroup> for HuskyComptime {
    fn upcast(&self) -> &(dyn husky_entity_semantics::EntityDefnQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn husky_entity_tree::EntityTreeDb> for HuskyComptime {
    fn upcast(&self) -> &(dyn husky_entity_tree::EntityTreeDb + 'static) {
        self
    }
}

impl Upcast<dyn husky_entity_tree::EntityTreeDb> for HuskyComptime {
    fn upcast(&self) -> &(dyn husky_entity_tree::EntityTreeDb + 'static) {
        self
    }
}

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
