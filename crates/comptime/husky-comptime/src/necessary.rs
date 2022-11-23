use crate::*;
use husky_linkage_table::{LinkageTable, ResolveLinkage};
use husky_vfs::{HasFileCache, HasWatcherPlace};
use place::SingleAssignPlace;
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
            // entity_route_store: self.entity_route_store.clone(),
            config: self.config.clone(),
        })
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

// impl StoreEntityRoute for HuskyComptime {
//     fn entity_route_store(&self) -> &EntityRouteStore {
//         &self.entity_route_store
//     }
// }

impl HasFileCache for HuskyComptime {
    fn cache(&self) -> &husky_vfs::HuskyFileCache {
        todo!()
    }
}

impl HasWatcherPlace for HuskyComptime {
    fn watcher_place_mut(&mut self) -> &mut SingleAssignPlace<husky_vfs::VfsWatcher> {
        todo!()
    }

    fn watcher_place(&self) -> &SingleAssignPlace<husky_vfs::VfsWatcher> {
        todo!()
    }
}
