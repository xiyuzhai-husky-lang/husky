use crate::*;
use bidashmap::BiDashMap;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

pub struct SourcePathJar(
    <SourcePath as salsa::storage::IngredientsFor>::Ingredients,
    <physical_path as salsa::storage::IngredientsFor>::Ingredients,
    BiDashMap<SourcePath, PhysicalPath>,
);

impl SourcePathJar {
    pub(crate) fn physical_path(
        &self,
        path: SourcePath,
        gen_physical_path: impl FnOnce() -> std::io::Result<PhysicalPath>,
    ) -> std::io::Result<PhysicalPath> {
        self.2
            .get_right_or_insert_with_result(path, gen_physical_path)
    }
}

impl salsa::storage::HasIngredientsFor<SourcePath> for SourcePathJar {
    fn ingredient(&self) -> &<SourcePath as salsa::storage::IngredientsFor>::Ingredients {
        &self.0
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <SourcePath as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.0
    }
}

impl salsa::storage::HasIngredientsFor<physical_path> for SourcePathJar {
    fn ingredient(&self) -> &<physical_path as salsa::storage::IngredientsFor>::Ingredients {
        &self.1
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <physical_path as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.1
    }
}

impl<'salsa_db> salsa::jar::Jar<'salsa_db> for SourcePathJar {
    type DynDb = dyn SourcePathDb + 'salsa_db;
    fn create_jar<DB>(routes: &mut salsa::routes::Routes<DB>) -> Self
    where
        DB: salsa::storage::JarFromJars<Self> + salsa::storage::DbWithJar<Self>,
    {
        let i0 = <SourcePath as salsa::storage::IngredientsFor>::create_ingredients(routes);
        let i1 = <physical_path as salsa::storage::IngredientsFor>::create_ingredients(routes);
        Self(i0, i1, Default::default())
    }
}
