use crate::*;

impl salsa::storage::IngredientsFor for Word {
    type Jar = WordJar;
    type Ingredients = salsa::interned::InternedIngredient<Word, __WordData>;
    fn create_ingredients<DB>(routes: &mut salsa::routes::Routes<DB>) -> Self::Ingredients
    where
        DB: salsa::storage::JarFromJars<Self::Jar>,
    {
        let index = routes.push(
            |jars| {
                let jar = <DB as salsa::storage::JarFromJars<Self::Jar>>::jar_from_jars(jars);
                <_ as salsa::storage::HasIngredientsFor<Self>>::ingredient(jar)
            },
            |jars| {
                let jar = <DB as salsa::storage::JarFromJars<Self::Jar>>::jar_from_jars_mut(jars);
                <_ as salsa::storage::HasIngredientsFor<Self>>::ingredient_mut(jar)
            },
        );
        salsa::interned::InternedIngredient::new(index, "Word")
    }
}

impl salsa::AsId for Word {
    fn as_id(self) -> salsa::Id {
        self.0
    }
    fn from_id(id: salsa::Id) -> Self {
        Word(id)
    }
}

impl<DB> salsa::salsa_struct::SalsaStructInDb<DB> for Word
where
    DB: ?Sized + salsa::DbWithJar<WordJar>,
{
    fn register_dependent_fn(_db: &DB, _index: salsa::routes::IngredientIndex) {}
}
impl ::salsa::DebugWithDb<<WordJar as salsa::jar::Jar<'_>>::DynDb> for Word {
    fn fmt(
        &self,
        f: &mut ::std::fmt::Formatter<'_>,
        _db: &<WordJar as salsa::jar::Jar<'_>>::DynDb,
        _include_all_fields: bool,
    ) -> ::std::fmt::Result {
        #[allow(unused_imports)]
        use ::salsa::debug::helper::Fallback;
        let mut debug_struct = &mut f.debug_struct("Word");
        debug_struct = debug_struct.field("[salsa id]", &self.0.as_u32());
        debug_struct =
            debug_struct.field(
                "data",
                &::salsa::debug::helper::SalsaDebug::<
                    String,
                    <WordJar as salsa::jar::Jar<'_>>::DynDb,
                >::salsa_debug(
                    #[allow(clippy::needless_borrow)]
                    &self.data(_db),
                    _db,
                    _include_all_fields,
                ),
            );
        debug_struct.finish()
    }
}
