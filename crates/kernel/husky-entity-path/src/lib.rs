#![feature(trait_upcasting)]
mod accessibility;
mod ancestry;
mod db;
mod error;
mod menu;
#[cfg(test)]
mod tests;
mod utils;

pub use accessibility::*;
pub use db::*;
pub use error::*;
pub use menu::*;

use ancestry::*;
use husky_package_path::*;
use husky_toolchain::Toolchain;
use husky_word::{Identifier, WordJar};
use optional::Optioned;
use salsa::DbWithJar;
use utils::*;

#[salsa::jar(db = EntityPathDb)]
pub struct EntityPathJar(EntityPath, apparent_ancestry, entity_path_menu);

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub struct EntityPath(salsa::Id);

#[doc = r" Internal struct used for interned item"]
#[derive(Eq, PartialEq, Hash, Clone)]
pub struct __EntityPathData {
    data: EntityPathData,
}
impl salsa::storage::IngredientsFor for EntityPath {
    type Jar = EntityPathJar;
    type Ingredients = salsa::interned::InternedIngredient<EntityPath, __EntityPathData>;
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
        salsa::interned::InternedIngredient::new(index, "EntityPath")
    }
}
impl salsa::AsId for EntityPath {
    fn as_id(self) -> salsa::Id {
        self.0
    }
    fn from_id(id: salsa::Id) -> Self {
        EntityPath(id)
    }
}
impl EntityPath {
    pub fn data(self, db: &<EntityPathJar as salsa::jar::Jar<'_>>::DynDb) -> EntityPathData {
        let (jar, runtime) = <_ as salsa::storage::HasJar<EntityPathJar>>::jar(db);
        let ingredients =
            <EntityPathJar as salsa::storage::HasIngredientsFor<EntityPath>>::ingredient(jar);
        std::clone::Clone::clone(&ingredients.data(runtime, self).data)
    }
    pub fn new(db: &<EntityPathJar as salsa::jar::Jar<'_>>::DynDb, data: EntityPathData) -> Self {
        let (jar, runtime) = <_ as salsa::storage::HasJar<EntityPathJar>>::jar(db);
        let ingredients =
            <EntityPathJar as salsa::storage::HasIngredientsFor<EntityPath>>::ingredient(jar);
        ingredients.intern(runtime, __EntityPathData { data })
    }
}
impl<DB> salsa::salsa_struct::SalsaStructInDb<DB> for EntityPath
where
    DB: ?Sized + salsa::DbWithJar<EntityPathJar>,
{
    fn register_dependent_fn(_db: &DB, _index: salsa::routes::IngredientIndex) {}
}
impl salsa::DebugWithDb<dyn EntityPathDb + '_> for EntityPath {
    fn fmt(
        &self,
        f: &mut ::std::fmt::Formatter<'_>,
        db: &<EntityPathJar as salsa::jar::Jar<'_>>::DynDb,
        include_all_fields: bool,
    ) -> ::std::fmt::Result {
        f.debug_struct("EntityPath")
            .field("[show]", &self.show(db))
            .field(
                "[crate]",
                &self.crate_path(db).debug_with(db, include_all_fields),
            )
            .finish()
    }
}

impl<Db> salsa::DebugWithDb<Db> for EntityPath
where
    Db: EntityPathDb,
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        self.fmt(f, db as &dyn EntityPathDb, include_all_fields)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum EntityPathData {
    CrateRoot(CratePath),
    Childpath {
        parent: EntityPath,
        ident: Identifier,
    },
}

impl EntityPath {
    pub fn new_crate_root(
        db: &dyn EntityPathDb,
        package_path: PackagePath,
        crate_kind: CrateKind,
    ) -> Self {
        db.it_entity_path(EntityPathData::CrateRoot(CratePath::new(
            db,
            package_path,
            crate_kind,
        )))
    }

    pub(crate) fn child(self, db: &dyn EntityPathDb, ident: &str) -> Option<EntityPath> {
        db.it_child_entity_path(self, ident)
    }

    pub fn show(self, db: &dyn EntityPathDb) -> String {
        match self.data(db) {
            EntityPathData::CrateRoot(crate_path) => "crate".to_owned(),
            EntityPathData::Childpath { parent, ident } => {
                format!("{}::{}", parent.show(db), db.dt_ident(ident))
            }
        }
    }

    pub fn crate_path(self, db: &dyn EntityPathDb) -> CratePath {
        apparent_ancestry(db, self).crate_path()
    }
}
