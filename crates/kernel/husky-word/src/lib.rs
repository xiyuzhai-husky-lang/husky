mod db;
mod ident;
mod interned;
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
    fn data<'db>(self, db: &'db <WordJar as salsa::jar::Jar<'_>>::DynDb) -> &'db String {
        let (jar, runtime) = <_ as salsa::storage::HasJar<WordJar>>::jar(db);
        let ingredients = <WordJar as salsa::storage::HasIngredientsFor<Word>>::ingredient(jar);
        &ingredients.data(runtime, self).data
    }

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
