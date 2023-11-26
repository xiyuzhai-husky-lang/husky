use crate::*;

pub struct Db {}

impl Db {
    pub fn jars(&self) -> (&Jars, &Runtime) {
        todo!()
    }

    /// Gets mutable access to the jars. This will trigger a new revision
    /// and it will also cancel any ongoing work in the current revision.
    pub fn jars_mut(&mut self) -> (&mut Jars, &mut Runtime) {
        todo!()
    }

    pub fn runtime(&self) -> &crate::Runtime {
        self.storage.runtime()
    }
    pub fn runtime_mut(&mut self) -> &mut crate::Runtime {
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
}
