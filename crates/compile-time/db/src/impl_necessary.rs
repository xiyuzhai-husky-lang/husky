use infer_total::InferQueryGroup;
use upcast::Upcast;

use crate::*;

impl fmt::Debug for HuskyLangCompileTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HuskyLangDatabase").finish()
    }
}

impl salsa::Database for HuskyLangCompileTime {}

impl salsa::ParallelDatabase for HuskyLangCompileTime {
    fn snapshot(&self) -> salsa::Snapshot<HuskyLangCompileTime> {
        salsa::Snapshot::new(HuskyLangCompileTime {
            storage: self.storage.snapshot(),
            file_unique_allocator: self.file_unique_allocator.clone(),
            word_unique_allocator: self.word_unique_allocator.clone(),
            scope_unique_allocator: self.scope_unique_allocator.clone(),
            live_docs: self.live_docs.clone(),
            vc: self.vc.clone(),
            features: self.features.clone(),
        })
    }
}

impl Default for HuskyLangCompileTime {
    fn default() -> Self {
        Self {
            storage: Default::default(),
            file_unique_allocator: file::new_file_unique_allocator(),
            word_unique_allocator: word::new_word_unique_allocator(),
            scope_unique_allocator: entity_route::new_scope_unique_allocator(),
            live_docs: Default::default(),
            vc: EntityVersionControl::new(),
            features: feature::new_feature_unique_allocator(),
        }
    }
}

impl AllocateUniqueFile for HuskyLangCompileTime {
    fn file_unique_allocator(&self) -> &file::UniqueFileAllocator {
        &self.file_unique_allocator
    }
}

impl InternWord for HuskyLangCompileTime {
    fn word_allocator(&self) -> &word::WordAllocator {
        &self.word_unique_allocator
    }
}

impl LiveFiles for HuskyLangCompileTime {
    fn get_live_files(&self) -> &ARwLock<HashMap<file::FilePtr, ARwLock<String>>> {
        &self.live_docs
    }

    fn did_change_source(&mut self, id: file::FilePtr) {
        file::FileContentQuery.in_db_mut(self).invalidate(&id);
    }
}

impl FileQueryGroup for HuskyLangCompileTime {}

impl AllocateUniqueScope for HuskyLangCompileTime {
    fn scope_unique_allocator(&self) -> &entity_route::ScopeInterner {
        &self.scope_unique_allocator
    }
}

impl TokenQueryGroup for HuskyLangCompileTime {}

impl ScopeQueryGroup for HuskyLangCompileTime {}

impl AstQueryGroup for HuskyLangCompileTime {}

impl Upcast<dyn InferQueryGroup> for HuskyLangCompileTime {
    fn upcast(&self) -> &(dyn infer_total::InferQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn semantics_entity::EntityQueryGroup> for HuskyLangCompileTime {
    fn upcast(&self) -> &(dyn semantics_entity::EntityQueryGroup + 'static) {
        self
    }
}

impl ControlEntityVersion for HuskyLangCompileTime {
    fn entity_vc(&self) -> &vc::VersionControl<EntityRoutePtr, EntityKind> {
        &self.vc
    }
}

impl AllocateUniqueFeature for HuskyLangCompileTime {
    fn features(&self) -> &feature::FeatureUniqueAllocator {
        &self.features
    }
}

impl Upcast<dyn scope_query::EntityRouteSalsaQueryGroup> for HuskyLangCompileTime {
    fn upcast(&self) -> &(dyn scope_query::EntityRouteSalsaQueryGroup + 'static) {
        self
    }
}

impl infer_ty::InferTyQueryGroup for HuskyLangCompileTime {}

impl infer_contract::InferContractQueryGroup for HuskyLangCompileTime {}

impl infer_total::InferQueryGroup for HuskyLangCompileTime {}
