#![feature(trait_upcasting)]
mod crate_path;
mod db;
mod error;
mod jar;
mod menu;
mod name;
#[cfg(test)]
mod tests;

pub use crate_path::*;
pub use db::*;
pub use error::*;
pub use jar::*;
pub use menu::*;

use ::salsa::DebugWithDb;
use husky_absolute_path::{AbsolutePath, AbsolutePathResult};
use husky_toolchain::{PublishedToolchain, Toolchain};
use husky_word::Identifier;
use name::package_name;
use semver::Version;
use std::path::{Path, PathBuf};
use url::Url;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PackagePathData {
    PublishedToolchain {
        ident: Identifier,
        toolchain: PublishedToolchain,
    },
    Global {
        ident: Identifier,
        version: Version,
    },
    Local {
        path: AbsolutePath,
    },
    Git {
        url: Url,
    },
}

// #[salsa::interned(jar = PackagePathJar)]
// pub struct PackagePath {
//     #[return_ref]
//     pub data: PackagePathData,
// }

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub struct PackagePath(salsa::Id);

impl PackagePath {
    pub fn ident(self, db: &dyn PackagePathDb) -> Identifier {
        match self.data(db) {
            PackagePathData::PublishedToolchain { ident, .. }
            | PackagePathData::Global { ident, .. } => *ident,
            PackagePathData::Local { .. } => todo!(),
            PackagePathData::Git { .. } => todo!(),
        }
    }

    pub fn new_local(db: &dyn PackagePathDb, path: &Path) -> AbsolutePathResult<Self> {
        Ok(PackagePath::new(
            db,
            PackagePathData::Local {
                path: AbsolutePath::new(path)?,
            },
        ))
    }
}

#[doc = r" Internal struct used for interned item"]
#[derive(Eq, PartialEq, Hash, Clone)]
pub struct __PackagePathData {
    data: PackagePathData,
}
impl salsa::storage::IngredientsFor for PackagePath {
    type Jar = PackagePathJar;
    type Ingredients = salsa::interned::InternedIngredient<PackagePath, __PackagePathData>;
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
        salsa::interned::InternedIngredient::new(index, "PackagePath")
    }
}
impl salsa::AsId for PackagePath {
    fn as_id(self) -> salsa::Id {
        self.0
    }
    fn from_id(id: salsa::Id) -> Self {
        PackagePath(id)
    }
}
impl PackagePath {
    pub fn data<'db>(
        self,
        db: &'db <PackagePathJar as salsa::jar::Jar<'_>>::DynDb,
    ) -> &'db PackagePathData {
        let (jar, runtime) = <_ as salsa::storage::HasJar<PackagePathJar>>::jar(db);
        let ingredients =
            <PackagePathJar as salsa::storage::HasIngredientsFor<PackagePath>>::ingredient(jar);
        &ingredients.data(runtime, self).data
    }
    pub fn new(db: &<PackagePathJar as salsa::jar::Jar<'_>>::DynDb, data: PackagePathData) -> Self {
        let (jar, runtime) = <_ as salsa::storage::HasJar<PackagePathJar>>::jar(db);
        let ingredients =
            <PackagePathJar as salsa::storage::HasIngredientsFor<PackagePath>>::ingredient(jar);
        ingredients.intern(runtime, __PackagePathData { data })
    }
}
impl<DB> salsa::salsa_struct::SalsaStructInDb<DB> for PackagePath
where
    DB: ?Sized + salsa::DbWithJar<PackagePathJar>,
{
    fn register_dependent_fn(_db: &DB, _index: salsa::routes::IngredientIndex) {}
}
impl DebugWithDb<dyn PackagePathDb + '_> for PackagePath {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn PackagePathDb,
        include_all_fields: bool,
    ) -> ::std::fmt::Result {
        match self.data(db) {
            PackagePathData::PublishedToolchain { ident, toolchain } => f
                .debug_struct("Builtin")
                .field("ident", &ident.data(db))
                .field("toolchain", &toolchain.debug(db))
                .finish(),
            PackagePathData::Global { ident, version } => f
                .debug_struct("Glocal")
                .field("ident", ident)
                .field("version", version)
                .finish(),
            PackagePathData::Local { path } => f
                .debug_struct("Local")
                .field("ident", &self.ident(db).debug_with(db, include_all_fields))
                .field("path", path)
                .finish(),
            PackagePathData::Git { url } => f
                .debug_struct("Git")
                .field("ident", &self.ident(db).debug_with(db, include_all_fields))
                .field("url", url)
                .finish(),
        }
    }
}
