use husky_salsa_log_utils::HasLogger;

use crate::storage::{HasJars, HasJarsDyn};

pub struct TestDb {
    storage: crate::Storage<Self>,
}

impl HasLogger for TestDb {
    fn logger(&self) -> &husky_salsa_log_utils::Logger {
        todo!()
    }
}

pub struct TestJars {}

impl crate::database::AsSalsaDatabase for TestDb {
    fn as_salsa_database(&self) -> &dyn crate::Database {
        self
    }
}

impl crate::Database for TestDb {}

impl HasJars for TestDb {
    type Jars = TestJars;

    fn jars(&self) -> (&Self::Jars, &crate::Runtime) {
        todo!()
    }

    fn jars_mut(&mut self) -> (&mut Self::Jars, &mut crate::Runtime) {
        todo!()
    }

    fn initialize_jars(jars: &mut Self::Jars, routes: &mut crate::routes::Routes<Self>) {
        todo!()
    }
}

impl HasJarsDyn for TestDb {
    fn runtime(&self) -> &crate::Runtime {
        self.storage.runtime()
    }
    fn runtime_mut(&mut self) -> &mut crate::Runtime {
        self.storage.runtime_mut()
    }
    fn maybe_changed_after(
        &self,
        input: crate::key::DependencyIndex,
        revision: crate::Revision,
    ) -> bool {
        let ingredient = self.storage.ingredient(input.ingredient_index());
        ingredient.maybe_changed_after(self, input, revision)
    }
    fn cycle_recovery_strategy(
        &self,
        ingredient_index: crate::IngredientIndex,
    ) -> crate::cycle::CycleRecoveryStrategy {
        let ingredient = self.storage.ingredient(ingredient_index);
        ingredient.cycle_recovery_strategy()
    }
    fn origin(
        &self,
        index: crate::DatabaseKeyIndex,
    ) -> Option<crate::runtime::local_state::QueryOrigin> {
        let ingredient = self.storage.ingredient(index.ingredient_index());
        ingredient.origin(index.key_index())
    }
    fn mark_validated_output(
        &self,
        executor: crate::DatabaseKeyIndex,
        output: crate::key::DependencyIndex,
    ) {
        let ingredient = self.storage.ingredient(output.ingredient_index());
        ingredient.mark_validated_output(self, executor, output.key_index());
    }
    fn remove_stale_output(
        &self,
        executor: crate::DatabaseKeyIndex,
        stale_output: crate::key::DependencyIndex,
    ) {
        let ingredient = self.storage.ingredient(stale_output.ingredient_index());
        ingredient.remove_stale_output(self, executor, stale_output.key_index());
    }
    fn salsa_struct_deleted(&self, ingredient: crate::IngredientIndex, id: crate::Id) {
        let ingredient = self.storage.ingredient(ingredient);
        ingredient.salsa_struct_deleted(self, id);
    }
    fn fmt_index(
        &self,
        index: crate::key::DependencyIndex,
        fmt: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let ingredient = self.storage.ingredient(index.ingredient_index());
        ingredient.fmt_index(index.key_index(), fmt)
    }
}

impl<Jar> crate::storage::HasJar<Jar> for TestDb {
    fn jar(&self) -> (&Jar, &crate::Runtime) {
        todo!()
    }

    fn jar_mut(&mut self) -> (&mut Jar, &mut crate::Runtime) {
        todo!()
    }
}

pub trait TestJar: for<'db> crate::jar::Jar<'db> {
    const n: usize;

    fn cast_test_db<'db>(db: &'db TestDb) -> &'db <Self as crate::jar::Jar<'db>>::DynDb;
}

impl<Jar> crate::storage::DbWithJar<Jar> for TestDb {
    fn as_jar_db<'db>(&'db self) -> &'db <Jar as crate::jar::Jar<'db>>::DynDb
    where
        Jar: crate::jar::Jar<'db>,
    {
        todo!()
        // Jar::cast_test_db(self)
    }
}
