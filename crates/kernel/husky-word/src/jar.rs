use crate::*;
use once_cell::sync::OnceCell;
use std::sync::Arc;

pub struct WordJar(
    <Word as salsa::storage::IngredientsFor>::Ingredients,
    Arc<OnceCell<WordMenu>>,
);

impl salsa::storage::HasIngredientsFor<Word> for WordJar {
    fn ingredient(&self) -> &<Word as salsa::storage::IngredientsFor>::Ingredients {
        &self.0
    }
    fn ingredient_mut(&mut self) -> &mut <Word as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.0
    }
}
impl<'salsa_db> salsa::jar::Jar<'salsa_db> for WordJar {
    type DynDb = dyn WordDb + 'salsa_db;
    fn create_jar<DB>(routes: &mut salsa::routes::Routes<DB>) -> Self
    where
        DB: salsa::storage::JarFromJars<Self> + salsa::storage::DbWithJar<Self>,
    {
        let i0 = <Word as salsa::storage::IngredientsFor>::create_ingredients(routes);
        Self(i0, Default::default())
    }
}

impl WordJar {
    pub(crate) fn word_menu_cell(&self) -> &OnceCell<WordMenu> {
        &self.1
    }
}
