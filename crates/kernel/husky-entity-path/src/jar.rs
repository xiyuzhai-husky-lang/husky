use crate::*;
use ancestry::apparent_ancestry;

pub struct EntityPathJar(
    <EntityPath as salsa::storage::IngredientsFor>::Ingredients,
    <apparent_ancestry as salsa::storage::IngredientsFor>::Ingredients,
    pub(crate) EntityPathMenuPlace,
);

impl EntityPathJar {
    pub(crate) fn entity_path_menu_place(&self) -> &EntityPathMenuPlace {
        &self.2
    }
}

impl salsa::storage::HasIngredientsFor<EntityPath> for EntityPathJar {
    fn ingredient(&self) -> &<EntityPath as salsa::storage::IngredientsFor>::Ingredients {
        &self.0
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <EntityPath as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.0
    }
}

impl salsa::storage::HasIngredientsFor<apparent_ancestry> for EntityPathJar {
    fn ingredient(&self) -> &<apparent_ancestry as salsa::storage::IngredientsFor>::Ingredients {
        &self.1
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <apparent_ancestry as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.1
    }
}

impl<'salsa_db> salsa::jar::Jar<'salsa_db> for EntityPathJar {
    type DynDb = dyn EntityPathDb + 'salsa_db;
    fn create_jar<DB>(routes: &mut salsa::routes::Routes<DB>) -> Self
    where
        DB: salsa::storage::JarFromJars<Self> + salsa::storage::DbWithJar<Self>,
    {
        let i0 = <EntityPath as salsa::storage::IngredientsFor>::create_ingredients(routes);
        let i1 = <apparent_ancestry as salsa::storage::IngredientsFor>::create_ingredients(routes);
        Self(i0, i1, Default::default())
    }
}
