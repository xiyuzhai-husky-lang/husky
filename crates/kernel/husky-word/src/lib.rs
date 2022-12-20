mod db;
mod ident;
mod jar;
mod menu;
mod style;
mod tests;

pub use db::*;
pub use ident::*;
pub use jar::*;
pub use menu::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub struct Word(salsa::Id);

#[doc = r" Internal struct used for interned item"]
#[derive(Eq, PartialEq, Hash, Clone)]
pub struct __WordData {
    data: String,
}
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
impl Word {
    fn data<'db>(self, db: &'db <WordJar as salsa::jar::Jar<'_>>::DynDb) -> &'db String {
        let (jar, runtime) = <_ as salsa::storage::HasJar<WordJar>>::jar(db);
        let ingredients = <WordJar as salsa::storage::HasIngredientsFor<Word>>::ingredient(jar);
        &ingredients.data(runtime, self).data
    }
    pub fn new(db: &<WordJar as salsa::jar::Jar<'_>>::DynDb, data: String) -> Self {
        let (jar, runtime) = <_ as salsa::storage::HasJar<WordJar>>::jar(db);
        let ingredients = <WordJar as salsa::storage::HasIngredientsFor<Word>>::ingredient(jar);
        ingredients.intern(runtime, __WordData { data })
    }
}
impl<DB> salsa::salsa_struct::SalsaStructInDb<DB> for Word
where
    DB: ?Sized + salsa::DbWithJar<WordJar>,
{
    fn register_dependent_fn(_db: &DB, _index: salsa::routes::IngredientIndex) {}
}

impl ::salsa::DebugWithDb<dyn WordDb + '_> for Word {
    fn fmt(
        &self,
        f: &mut ::std::fmt::Formatter<'_>,
        db: &dyn WordDb,
        _include_all_fields: bool,
    ) -> ::std::fmt::Result {
        f.debug_tuple("Word").field(&self.data(db)).finish()
    }
}

impl<Db: WordDb> ::salsa::DebugWithDb<Db> for Word {
    fn fmt(
        &self,
        f: &mut ::std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> ::std::fmt::Result {
        self.fmt(f, db as &dyn WordDb, include_all_fields)
    }
}

// #[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
// pub struct Word(salsa::Id);

// #[doc = r" Internal struct used for interned item"]
// #[derive(Eq, PartialEq, Hash, Clone)]
// pub struct __WordData {
//     data: String,
// }

impl std::borrow::Borrow<str> for __WordData {
    fn borrow(&self) -> &str {
        &self.data
    }
}

impl<'a> From<&'a str> for __WordData {
    fn from(value: &'a str) -> Self {
        Self { data: value.into() }
    }
}

impl Word {
    fn from_owned(db: &<WordJar as salsa::jar::Jar<'_>>::DynDb, data: String) -> Self {
        let (jar, runtime) = <_ as salsa::storage::HasJar<WordJar>>::jar(db);
        let ingredients = <WordJar as salsa::storage::HasIngredientsFor<Word>>::ingredient(jar);
        ingredients.intern(runtime, __WordData { data })
    }

    fn from_ref(db: &<WordJar as salsa::jar::Jar<'_>>::DynDb, data: &str) -> Self {
        let (jar, runtime) = <_ as salsa::storage::HasJar<WordJar>>::jar(db);
        let ingredients = <WordJar as salsa::storage::HasIngredientsFor<Word>>::ingredient(jar);
        ingredients.intern_borrowed(runtime, data)
    }
}

// impl salsa::storage::IngredientsFor for Word {
//     type Jar = WordJar;
//     type Ingredients = salsa::interned::InternedIngredient<Word, __WordData>;
//     fn create_ingredients<DB>(routes: &mut salsa::routes::Routes<DB>) -> Self::Ingredients
//     where
//         DB: salsa::storage::JarFromJars<Self::Jar>,
//     {
//         let index = routes.push(
//             |jars| {
//                 let jar = <DB as salsa::storage::JarFromJars<Self::Jar>>::jar_from_jars(jars);
//                 <_ as salsa::storage::HasIngredientsFor<Self>>::ingredient(jar)
//             },
//             |jars| {
//                 let jar = <DB as salsa::storage::JarFromJars<Self::Jar>>::jar_from_jars_mut(jars);
//                 <_ as salsa::storage::HasIngredientsFor<Self>>::ingredient_mut(jar)
//             },
//         );
//         salsa::interned::InternedIngredient::new(index, "Word")
//     }
// }

// impl salsa::AsId for Word {
//     fn as_id(self) -> salsa::Id {
//         self.0
//     }
//     fn from_id(id: salsa::Id) -> Self {
//         Word(id)
//     }
// }

// impl<DB> salsa::salsa_struct::SalsaStructInDb<DB> for Word
// where
//     DB: ?Sized + salsa::DbWithJar<WordJar>,
// {
//     fn register_dependent_fn(_db: &DB, _index: salsa::routes::IngredientIndex) {}
// }
// impl ::salsa::DebugWithDb<<WordJar as salsa::jar::Jar<'_>>::DynDb> for Word {
//     fn fmt(
//         &self,
//         f: &mut ::std::fmt::Formatter<'_>,
//         _db: &<WordJar as salsa::jar::Jar<'_>>::DynDb,
//         _include_all_fields: bool,
//     ) -> ::std::fmt::Result {
//         #[allow(unused_imports)]
//         use ::salsa::debug::helper::Fallback;
//         let mut debug_struct = &mut f.debug_struct("Word");
//         debug_struct = debug_struct.field("[salsa id]", &self.0.as_u32());
//         debug_struct =
//             debug_struct.field(
//                 "data",
//                 &::salsa::debug::helper::SalsaDebug::<
//                     String,
//                     <WordJar as salsa::jar::Jar<'_>>::DynDb,
//                 >::salsa_debug(
//                     #[allow(clippy::needless_borrow)]
//                     &self.data(_db),
//                     _db,
//                     _include_all_fields,
//                 ),
//             );
//         debug_struct.finish()
//     }
// }
