use crate::*;
use husky_debugger_protocol::*;
use infer_total::InferQueryGroup;
use linkage_table::{LinkageSourceTable, ResolveLinkage};
use semantics_entity::{EntityRouteStore, StoreEntityRoute};
use upcast::Upcast;
use vm::{AnyValueDyn, InterpreterQueryGroup};

impl fmt::Debug for HuskyCompileTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HuskyLangCompileTime").finish()
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
            features: self.features.clone(),
            linkage_table: self.linkage_table.clone(),
            entity_route_store: self.entity_route_store.clone(),
        })
    }
}

impl Default for HuskyCompileTime {
    fn default() -> Self {
        let live_docs = Default::default();
        let scope_unique_allocator = entity_route::new_entity_route_interner();
        let features = feature::new_feature_unique_allocator();
        let entity_route_store = Default::default();
        let linkage_table = Default::default();
        Self {
            storage: Default::default(),
            file_unique_allocator: file::new_file_unique_allocator(),
            word_unique_allocator: word::new_word_interner(),
            scope_unique_allocator,
            live_docs,
            features,
            linkage_table,
            entity_route_store,
        }
    }
}

impl AllocateUniqueFile for HuskyCompileTime {
    fn file_interner(&self) -> &file::FileInterner {
        &self.file_unique_allocator
    }
}

impl InternWord for HuskyCompileTime {
    fn word_allocator(&self) -> &word::WordAllocator {
        &self.word_unique_allocator
    }
}

impl LiveFiles for HuskyCompileTime {
    fn get_live_files(&self) -> &ARwLock<IndexMap<file::FilePtr, ARwLock<String>>> {
        &self.live_docs
    }

    fn did_change_source(&mut self, id: file::FilePtr) {
        file::FileContentQuery.in_db_mut(self).invalidate(&id);
    }
}

impl FileQueryGroup for HuskyCompileTime {}

impl AllocateUniqueScope for HuskyCompileTime {
    fn scope_unique_allocator(&self) -> &entity_route::EntityRouteInterner {
        &self.scope_unique_allocator
    }
}

impl TokenQueryGroup for HuskyCompileTime {}

impl EntitySyntaxQueryGroup for HuskyCompileTime {}

impl AstQueryGroup for HuskyCompileTime {}

impl Upcast<dyn InferQueryGroup> for HuskyCompileTime {
    fn upcast(&self) -> &(dyn infer_total::InferQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn semantics_entity::EntityDefnQueryGroup> for HuskyCompileTime {
    fn upcast(&self) -> &(dyn semantics_entity::EntityDefnQueryGroup + 'static) {
        self
    }
}

impl AllocateUniqueFeature for HuskyCompileTime {
    fn features(&self) -> &feature::FeatureUniqueAllocator {
        &self.features
    }
}

impl Upcast<dyn entity_syntax::EntitySyntaxSalsaQueryGroup> for HuskyCompileTime {
    fn upcast(&self) -> &(dyn entity_syntax::EntitySyntaxSalsaQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn entity_syntax::EntitySyntaxQueryGroup> for HuskyCompileTime {
    fn upcast(&self) -> &(dyn entity_syntax::EntitySyntaxQueryGroup + 'static) {
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
    fn linkage_table(&self) -> &LinkageSourceTable {
        &self.linkage_table
    }
}

impl InterpreterQueryGroup for HuskyCompileTime {
    fn entity_opt_instruction_sheet_by_uid(
        &self,
        uid: vm::EntityUid,
    ) -> Option<Arc<vm::InstructionSheet>> {
        let entity_route = self.entity_route_by_uid(uid);
        self.entity_instruction_sheet(entity_route)
    }

    fn visualize<'temp, 'eval>(
        &self,
        _ty: EntityRoutePtr,
        _value: &(dyn AnyValueDyn<'eval> + 'temp),
    ) -> VisualProps {
        panic!("can only visualize in HuskyRuntime")
    }
}

impl Upcast<dyn InterpreterQueryGroup> for HuskyCompileTime {
    fn upcast(&self) -> &(dyn InterpreterQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn InstructionGenQueryGroup> for HuskyCompileTime {
    fn upcast(&self) -> &(dyn InstructionGenQueryGroup + 'static) {
        self
    }
}

impl StoreEntityRoute for HuskyCompileTime {
    fn entity_route_store(&self) -> &EntityRouteStore {
        &self.entity_route_store
    }
}
