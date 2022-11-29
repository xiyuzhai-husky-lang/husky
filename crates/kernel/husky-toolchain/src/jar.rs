use crate::*;
use once_cell::sync::OnceCell;
use std::sync::Arc;

pub struct ToolchainJar(
    <Toolchain as salsa::storage::IngredientsFor>::Ingredients,
    Arc<once_cell::sync::OnceCell<Toolchain>>,
);

impl ToolchainJar {
    pub fn toolchain_cell(&self) -> &OnceCell<Toolchain> {
        &self.1
    }
}

impl salsa::storage::HasIngredientsFor<Toolchain> for ToolchainJar {
    fn ingredient(&self) -> &<Toolchain as salsa::storage::IngredientsFor>::Ingredients {
        &self.0
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <Toolchain as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.0
    }
}
impl<'salsa_db> salsa::jar::Jar<'salsa_db> for ToolchainJar {
    type DynDb = dyn ToolchainDb + 'salsa_db;
    fn create_jar<DB>(routes: &mut salsa::routes::Routes<DB>) -> Self
    where
        DB: salsa::storage::JarFromJars<Self> + salsa::storage::DbWithJar<Self>,
    {
        let i0 = <Toolchain as salsa::storage::IngredientsFor>::create_ingredients(routes);
        Self(i0, Default::default())
    }
}
