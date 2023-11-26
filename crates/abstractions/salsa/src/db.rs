use husky_salsa_log_utils::HasLogger;

use crate::{jar::HasJarIndex, routes::Routes, Runtime, *};

pub struct Db {
    storage: crate::storage::Storage,
    logger: husky_salsa_log_utils::Logger,
}

pub trait HasDb<'a> {
    fn db(&self) -> &'a Db;
}

impl Db {
    /// here we use fn instead of impl FnOnce to avoid inlining to save compilation time
    pub fn new(initialize_jars: fn(&mut Jars, &mut Routes)) -> Self {
        Self {
            storage: crate::Storage::new(initialize_jars),
            logger: Default::default(),
        }
    }

    pub fn jars(&self) -> (&Jars, &Runtime) {
        todo!()
    }

    /// Gets mutable access to the jars. This will trigger a new revision
    /// and it will also cancel any ongoing work in the current revision.
    pub fn jars_mut(&mut self) -> (&mut Jars, &mut Runtime) {
        todo!()
    }

    pub fn jar<Jar>(&self) -> (&Jar, &Runtime)
    where
        Jar: HasJarIndex + 'static,
    {
        let (jars, runtime) = self.storage.jars();
        (jars.jar(), runtime)
    }

    pub fn jar_mut<Jar>(&mut self) -> (&mut Jar, &mut Runtime)
    where
        Jar: HasJarIndex + 'static,
    {
        let (jars, runtime) = self.storage.jars_mut();
        (jars.jar_mut(), runtime)
    }

    pub fn runtime(&self) -> &Runtime {
        self.storage.runtime()
    }
    pub fn runtime_mut(&mut self) -> &mut Runtime {
        self.storage.runtime_mut()
    }
    pub fn maybe_changed_after(
        &self,
        input: crate::key::DependencyIndex,
        revision: crate::Revision,
    ) -> bool {
        let ingredient = self.storage.ingredient(input.ingredient_index());
        ingredient.maybe_changed_after(self, input, revision)
    }
    pub fn cycle_recovery_strategy(
        &self,
        ingredient_index: crate::IngredientIndex,
    ) -> crate::cycle::CycleRecoveryStrategy {
        let ingredient = self.storage.ingredient(ingredient_index);
        ingredient.cycle_recovery_strategy()
    }
    pub fn origin(
        &self,
        index: crate::DatabaseKeyIndex,
    ) -> Option<crate::runtime::local_state::QueryOrigin> {
        let ingredient = self.storage.ingredient(index.ingredient_index());
        ingredient.origin(index.key_index())
    }
    pub fn mark_validated_output(
        &self,
        executor: crate::DatabaseKeyIndex,
        output: crate::key::DependencyIndex,
    ) {
        let ingredient = self.storage.ingredient(output.ingredient_index());
        ingredient.mark_validated_output(self, executor, output.key_index());
    }
    pub fn remove_stale_output(
        &self,
        executor: crate::DatabaseKeyIndex,
        stale_output: crate::key::DependencyIndex,
    ) {
        let ingredient = self.storage.ingredient(stale_output.ingredient_index());
        ingredient.remove_stale_output(self, executor, stale_output.key_index());
    }
    pub fn salsa_struct_deleted(&self, ingredient: crate::IngredientIndex, id: crate::Id) {
        let ingredient = self.storage.ingredient(ingredient);
        ingredient.salsa_struct_deleted(self, id);
    }
    pub fn fmt_index(
        &self,
        index: crate::key::DependencyIndex,
        fmt: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let ingredient = self.storage.ingredient(index.ingredient_index());
        ingredient.fmt_index(index.key_index(), fmt)
    }

    /// This function is invoked at key points in the salsa
    /// runtime. It permits the database to be customized and to
    /// inject logging or other custom behavior.
    ///
    /// By default, the event is logged at level debug using
    /// the standard `log` facade.
    pub fn salsa_event(&self, event: Event) {
        log::debug!("salsa_event: {:?}", event.debug(self));
    }

    /// A "synthetic write" causes the system to act *as though* some
    /// input of durability `durability` has changed. This is mostly
    /// useful for profiling scenarios.
    ///
    /// **WARNING:** Just like an ordinary write, this method triggers
    /// cancellation. If you invoke it while a snapshot exists, it
    /// will block until that snapshot is dropped -- if that snapshot
    /// is owned by the current thread, this could trigger deadlock.
    pub fn synthetic_write(&mut self, durability: Durability) {
        self.runtime_mut().report_tracked_write(durability);
    }

    /// Reports that the query depends on some state unknown to salsa.
    ///
    /// Queries which report untracked reads will be re-executed in the next
    /// revision.
    pub fn report_untracked_read(&self) {
        self.runtime().report_untracked_read();
    }
}

impl HasLogger for Db {
    fn logger(&self) -> &husky_salsa_log_utils::Logger {
        &self.logger
    }
}
