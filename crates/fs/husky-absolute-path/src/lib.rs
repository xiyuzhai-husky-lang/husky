#![feature(absolute_path)]

mod db;
mod error;

pub use db::*;
pub use error::*;

use husky_entity_path::EntityPathData;
use husky_package_path::PackagePathData;
use std::path::{Path, PathBuf};

#[salsa::jar(db=AbsolutePathDb)]
pub struct AbsolutePathJar(AbsolutePath);

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub struct AbsolutePath(salsa::Id);

#[doc = r" Internal struct used for interned item"]
#[derive(Eq, PartialEq, Hash, Clone)]
pub struct __AbsolutePathData {
    path: PathBuf,
}

impl salsa::storage::IngredientsFor for AbsolutePath {
    type Jar = AbsolutePathJar;
    type Ingredients = salsa::interned::InternedIngredient<AbsolutePath, __AbsolutePathData>;
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
        salsa::interned::InternedIngredient::new(index, "AbsolutePath")
    }
}
impl salsa::AsId for AbsolutePath {
    fn as_id(self) -> salsa::Id {
        self.0
    }
    fn from_id(id: salsa::Id) -> Self {
        AbsolutePath(id)
    }
}

impl AbsolutePath {
    /// this function is unsafe because its usage might result in untracked file read
    pub unsafe fn path<'db>(
        self,
        db: &'db <AbsolutePathJar as salsa::jar::Jar<'_>>::DynDb,
    ) -> &'db PathBuf {
        let (jar, runtime) = <_ as salsa::storage::HasJar<AbsolutePathJar>>::jar(db);
        let ingredients =
            <AbsolutePathJar as salsa::storage::HasIngredientsFor<AbsolutePath>>::ingredient(jar);
        &ingredients.data(runtime, self).path
    }

    fn new(db: &<AbsolutePathJar as salsa::jar::Jar<'_>>::DynDb, path: PathBuf) -> Self {
        let (jar, runtime) = <_ as salsa::storage::HasJar<AbsolutePathJar>>::jar(db);
        let ingredients =
            <AbsolutePathJar as salsa::storage::HasIngredientsFor<AbsolutePath>>::ingredient(jar);
        ingredients.intern(runtime, __AbsolutePathData { path })
    }
}

impl<DB> salsa::salsa_struct::SalsaStructInDb<DB> for AbsolutePath
where
    DB: ?Sized + salsa::DbWithJar<AbsolutePathJar>,
{
    fn register_dependent_fn(_db: &DB, _index: salsa::routes::IngredientIndex) {}
}

impl ::salsa::DebugWithDb<<AbsolutePathJar as salsa::jar::Jar<'_>>::DynDb> for AbsolutePath {
    fn fmt(
        &self,
        f: &mut ::std::fmt::Formatter<'_>,
        _db: &<AbsolutePathJar as salsa::jar::Jar<'_>>::DynDb,
        _include_all_fields: bool,
    ) -> ::std::fmt::Result {
        #[allow(unused_imports)]
        use ::salsa::debug::helper::Fallback;
        let mut debug_struct = &mut f.debug_struct("AbsolutePath");
        debug_struct = debug_struct.field("[salsa id]", &self.0.as_u32());
        debug_struct = debug_struct.field(
            "path",
            &::salsa::debug::helper::SalsaDebug::<
                PathBuf,
                <AbsolutePathJar as salsa::jar::Jar<'_>>::DynDb,
            >::salsa_debug(
                #[allow(clippy::needless_borrow)]
                &unsafe { self.path(_db) },
                _db,
                _include_all_fields,
            ),
        );
        debug_struct.finish()
    }
}

impl AbsolutePath {
    pub fn new_from_owned(db: &dyn AbsolutePathDb, path: PathBuf) -> AbsolutePathResult<Self> {
        Ok(Self::new(
            db,
            std::path::absolute(&path).map_err(|e| AbsolutePathError::FailToAbsolutize {
                path,
                error_message: e.to_string(),
            })?,
        ))
    }

    pub unsafe fn join(
        self,
        db: &dyn AbsolutePathDb,
        path: impl AsRef<Path>,
    ) -> AbsolutePathResult<Self> {
        Self::new_from_owned(db, self.path(db).join(path))
    }

    pub unsafe fn parent(
        self,
        db: &impl AsRef<dyn AbsolutePathDb>,
    ) -> AbsolutePathResult<Option<&Path>> {
        Ok(self.path(db.as_ref()).parent())
    }
}
