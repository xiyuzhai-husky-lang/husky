use crate::*;

pub struct TermJar(
    <Term as salsa::storage::IngredientsFor>::Ingredients,
    <TermCurryContext as salsa::storage::IngredientsFor>::Ingredients,
    TermMenuPlace,
);

impl TermJar {
    pub(crate) fn term_menu_place(&self) -> &TermMenuPlace {
        &self.2
    }
}

impl salsa::storage::HasIngredientsFor<Term> for TermJar {
    fn ingredient(&self) -> &<Term as salsa::storage::IngredientsFor>::Ingredients {
        &self.0
    }
    fn ingredient_mut(&mut self) -> &mut <Term as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.0
    }
}

impl salsa::storage::HasIngredientsFor<TermCurryContext> for TermJar {
    fn ingredient(&self) -> &<TermCurryContext as salsa::storage::IngredientsFor>::Ingredients {
        &self.1
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <TermCurryContext as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.1
    }
}

impl<'salsa_db> salsa::jar::Jar<'salsa_db> for TermJar {
    type DynDb = dyn TermDb + 'salsa_db;
    fn create_jar<DB>(routes: &mut salsa::routes::Routes<DB>) -> Self
    where
        DB: salsa::storage::JarFromJars<Self> + salsa::storage::DbWithJar<Self>,
    {
        let i0 = <Term as salsa::storage::IngredientsFor>::create_ingredients(routes);
        let i1 = <TermCurryContext as salsa::storage::IngredientsFor>::create_ingredients(routes);
        Self(i0, i1, Default::default())
    }
}
