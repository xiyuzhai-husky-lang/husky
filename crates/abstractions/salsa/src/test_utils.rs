use salsa::storage::{HasJars, HasJarsDyn};

pub struct TestDb {
    storage: salsa::Storage<Self>,
}

pub struct TestJars {}

impl salsa::database::AsSalsaDatabase for TestDb {
    fn as_salsa_database(&self) -> &dyn salsa::Database {
        self
    }
}

impl salsa::Database for TestDb {}

impl HasJars for TestDb {
    type Jars = TestJars;

    fn jars(&self) -> (&Self::Jars, &salsa::Runtime) {
        todo!()
    }

    fn jars_mut(&mut self) -> (&mut Self::Jars, &mut salsa::Runtime) {
        todo!()
    }

    fn initialize_jars(jars: &mut Self::Jars, routes: &mut salsa::routes::Routes<Self>) {
        todo!()
    }
}

impl HasJarsDyn for TestDb {
    fn runtime(&self) -> &salsa::Runtime {
        self.storage.runtime()
    }
    fn runtime_mut(&mut self) -> &mut salsa::Runtime {
        self.storage.runtime_mut()
    }
    fn maybe_changed_after(
        &self,
        input: salsa::key::DependencyIndex,
        revision: salsa::Revision,
    ) -> bool {
        let ingredient = self.storage.ingredient(input.ingredient_index());
        ingredient.maybe_changed_after(self, input, revision)
    }
    fn cycle_recovery_strategy(
        &self,
        ingredient_index: salsa::IngredientIndex,
    ) -> salsa::cycle::CycleRecoveryStrategy {
        let ingredient = self.storage.ingredient(ingredient_index);
        ingredient.cycle_recovery_strategy()
    }
    fn origin(
        &self,
        index: salsa::DatabaseKeyIndex,
    ) -> Option<salsa::runtime::local_state::QueryOrigin> {
        let ingredient = self.storage.ingredient(index.ingredient_index());
        ingredient.origin(index.key_index())
    }
    fn mark_validated_output(
        &self,
        executor: salsa::DatabaseKeyIndex,
        output: salsa::key::DependencyIndex,
    ) {
        let ingredient = self.storage.ingredient(output.ingredient_index());
        ingredient.mark_validated_output(self, executor, output.key_index());
    }
    fn remove_stale_output(
        &self,
        executor: salsa::DatabaseKeyIndex,
        stale_output: salsa::key::DependencyIndex,
    ) {
        let ingredient = self.storage.ingredient(stale_output.ingredient_index());
        ingredient.remove_stale_output(self, executor, stale_output.key_index());
    }
    fn salsa_struct_deleted(&self, ingredient: salsa::IngredientIndex, id: salsa::Id) {
        let ingredient = self.storage.ingredient(ingredient);
        ingredient.salsa_struct_deleted(self, id);
    }
    fn fmt_index(
        &self,
        index: salsa::key::DependencyIndex,
        fmt: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let ingredient = self.storage.ingredient(index.ingredient_index());
        ingredient.fmt_index(index.key_index(), fmt)
    }
}

impl<Jar> salsa::storage::HasJar<Jar> for TestDb {
    fn jar(&self) -> (&Jar, &salsa::Runtime) {
        todo!()
    }

    fn jar_mut(&mut self) -> (&mut Jar, &mut salsa::Runtime) {
        todo!()
    }
}

pub trait TestJar: for<'db> salsa::jar::Jar<'db> {
    const n: usize;

    fn cast_test_db<'db>(db: &'db TestDb) -> &'db <Self as salsa::jar::Jar<'db>>::DynDb;
}

impl<Jar> salsa::storage::DbWithJar<Jar> for TestDb {
    fn as_jar_db<'db>(&'db self) -> &'db <Jar as salsa::jar::Jar<'db>>::DynDb
    where
        Jar: salsa::jar::Jar<'db>,
    {
        todo!()
        // Jar::cast_test_db(self)
    }
}
