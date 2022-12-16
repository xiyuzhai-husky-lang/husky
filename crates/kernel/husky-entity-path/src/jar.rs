use crate::*;
use ancestry::ancestry;

pub struct EntityPathJar(
    <EntityPath as salsa::storage::IngredientsFor>::Ingredients,
    <entity_package as salsa::storage::IngredientsFor>::Ingredients,
    <is_builtin_entity as salsa::storage::IngredientsFor>::Ingredients,
    <ancestry as salsa::storage::IngredientsFor>::Ingredients,
    pub(crate) EntityPathMenuPlace,
);

impl EntityPathJar {
    pub(crate) fn entity_path_menu_place(&self) -> &EntityPathMenuPlace {
        &self.4
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
impl salsa::storage::HasIngredientsFor<entity_package> for EntityPathJar {
    fn ingredient(&self) -> &<entity_package as salsa::storage::IngredientsFor>::Ingredients {
        &self.1
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <entity_package as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.1
    }
}

impl salsa::storage::HasIngredientsFor<is_builtin_entity> for EntityPathJar {
    fn ingredient(&self) -> &<is_builtin_entity as salsa::storage::IngredientsFor>::Ingredients {
        &self.2
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <is_builtin_entity as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.2
    }
}

impl salsa::storage::HasIngredientsFor<ancestry> for EntityPathJar {
    fn ingredient(&self) -> &<ancestry as salsa::storage::IngredientsFor>::Ingredients {
        &self.3
    }
    fn ingredient_mut(&mut self) -> &mut <ancestry as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.3
    }
}

impl<'salsa_db> salsa::jar::Jar<'salsa_db> for EntityPathJar {
    type DynDb = dyn EntityPathDb + 'salsa_db;
    fn create_jar<DB>(routes: &mut salsa::routes::Routes<DB>) -> Self
    where
        DB: salsa::storage::JarFromJars<Self> + salsa::storage::DbWithJar<Self>,
    {
        let i0 = <EntityPath as salsa::storage::IngredientsFor>::create_ingredients(routes);
        let i1 = <entity_package as salsa::storage::IngredientsFor>::create_ingredients(routes);
        let i2 = <is_builtin_entity as salsa::storage::IngredientsFor>::create_ingredients(routes);
        let i3 = <ancestry as salsa::storage::IngredientsFor>::create_ingredients(routes);
        Self(i0, i1, i2, i3, Default::default())
    }
}
