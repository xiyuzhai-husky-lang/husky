use husky_absolute_path::AbsolutePath;
use husky_entity_path::EntityPath;
use husky_package_path::PackagePath;
use salsa::{input::InputIngredient, input_field::InputFieldIngredient, Durability};

use crate::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub struct File(salsa::Id);

#[derive(Debug, PartialEq, Eq)]
pub enum FileContent {
    NotExists,
    OnDisk(String),
    Live(String),
    Directory(Vec<AbsolutePath>),
    Err(VfsError),
}

impl File {
    pub(crate) fn text(self, db: &dyn VfsDb) -> VfsResult<&str> {
        self.content(db).text(self.path(db).as_ref())
    }
}

impl FileContent {
    pub(crate) fn text(&self, path: &Path) -> VfsResult<&str> {
        match self {
            FileContent::NotExists => Err(VfsError::FileNotExists(path.to_owned())),
            FileContent::OnDisk(text) | FileContent::Live(text) => Ok(text),
            FileContent::Directory(_) => todo!(),
            FileContent::Err(_) => todo!(),
        }
    }
}

impl File {
    pub(crate) fn new(
        db: &<crate::VfsJar as salsa::jar::Jar<'_>>::DynDb,
        path: AbsolutePath,
        content: FileContent,
        durability: Durability,
    ) -> Self {
        let (jar, runtime) = <_ as salsa::storage::HasJar<VfsJar>>::jar(db);
        let ingredients = <VfsJar as salsa::storage::HasIngredientsFor<File>>::ingredient(jar);
        let id = ingredients.2.new_input(runtime);
        ingredients.0.store_new(runtime, id, path, Durability::HIGH);
        ingredients.1.store_new(runtime, id, content, durability);
        id
    }

    pub(crate) fn path<'db>(
        self,
        __db: &'db <VfsJar as salsa::jar::Jar<'_>>::DynDb,
    ) -> AbsolutePath {
        let (__jar, __runtime) = <_ as salsa::storage::HasJar<VfsJar>>::jar(__db);
        let __ingredients = <VfsJar as salsa::storage::HasIngredientsFor<File>>::ingredient(__jar);
        __ingredients.0.fetch(__runtime, self).clone()
    }

    pub(crate) fn content<'db>(
        self,
        __db: &'db <VfsJar as salsa::jar::Jar<'_>>::DynDb,
    ) -> &'db FileContent {
        let (__jar, __runtime) = <_ as salsa::storage::HasJar<VfsJar>>::jar(__db);
        let __ingredients = <VfsJar as salsa::storage::HasIngredientsFor<File>>::ingredient(__jar);
        __ingredients.1.fetch(__runtime, self)
    }

    pub(crate) fn set_content<'db>(
        self,
        db: &'db mut <VfsJar as salsa::jar::Jar<'_>>::DynDb,
    ) -> salsa::setter::Setter<'db, File, FileContent> {
        let (__jar, __runtime) = <_ as salsa::storage::HasJar<VfsJar>>::jar_mut(db);
        let __ingredients =
            <VfsJar as salsa::storage::HasIngredientsFor<File>>::ingredient_mut(__jar);
        salsa::setter::Setter::new(__runtime, self, &mut __ingredients.1)
    }
}

impl salsa::storage::IngredientsFor for File {
    type Jar = VfsJar;
    type Ingredients = (
        InputFieldIngredient<File, AbsolutePath>,
        InputFieldIngredient<File, FileContent>,
        InputIngredient<File>,
    );
    fn create_ingredients<DB>(routes: &mut salsa::routes::Routes<DB>) -> Self::Ingredients
    where
        DB: salsa::DbWithJar<Self::Jar> + salsa::storage::JarFromJars<Self::Jar>,
    {
        (
            {
                let index = routes.push(
                    |jars| {
                        let jar =
                            <DB as salsa::storage::JarFromJars<Self::Jar>>::jar_from_jars(jars);
                        let ingredients =
                            <_ as salsa::storage::HasIngredientsFor<Self>>::ingredient(jar);
                        &ingredients.0
                    },
                    |jars| {
                        let jar =
                            <DB as salsa::storage::JarFromJars<Self::Jar>>::jar_from_jars_mut(jars);
                        let ingredients =
                            <_ as salsa::storage::HasIngredientsFor<Self>>::ingredient_mut(jar);
                        &mut ingredients.0
                    },
                );
                InputFieldIngredient::new(index, "path")
            },
            {
                let index = routes.push(
                    |jars| {
                        let jar =
                            <DB as salsa::storage::JarFromJars<Self::Jar>>::jar_from_jars(jars);
                        let ingredients =
                            <_ as salsa::storage::HasIngredientsFor<Self>>::ingredient(jar);
                        &ingredients.1
                    },
                    |jars| {
                        let jar =
                            <DB as salsa::storage::JarFromJars<Self::Jar>>::jar_from_jars_mut(jars);
                        let ingredients =
                            <_ as salsa::storage::HasIngredientsFor<Self>>::ingredient_mut(jar);
                        &mut ingredients.1
                    },
                );
                InputFieldIngredient::new(index, "content")
            },
            {
                let index = routes.push(
                    |jars| {
                        let jar =
                            <DB as salsa::storage::JarFromJars<Self::Jar>>::jar_from_jars(jars);
                        let ingredients =
                            <_ as salsa::storage::HasIngredientsFor<Self>>::ingredient(jar);
                        &ingredients.2
                    },
                    |jars| {
                        let jar =
                            <DB as salsa::storage::JarFromJars<Self::Jar>>::jar_from_jars_mut(jars);
                        let ingredients =
                            <_ as salsa::storage::HasIngredientsFor<Self>>::ingredient_mut(jar);
                        &mut ingredients.2
                    },
                );
                InputIngredient::new(index, "HuskyFile")
            },
        )
    }
}
impl salsa::AsId for File {
    fn as_id(self) -> salsa::Id {
        self.0
    }
    fn from_id(id: salsa::Id) -> Self {
        File(id)
    }
}
impl ::salsa::DebugWithDb<<VfsJar as salsa::jar::Jar<'_>>::DynDb> for File {
    fn fmt(
        &self,
        f: &mut ::std::fmt::Formatter<'_>,
        _db: &<VfsJar as salsa::jar::Jar<'_>>::DynDb,
        _include_all_fields: bool,
    ) -> ::std::fmt::Result {
        #[allow(unused_imports)]
        use ::salsa::debug::helper::Fallback;
        let mut debug_struct = &mut f.debug_struct("HuskyFile");
        debug_struct = debug_struct.field("[salsa id]", &self.0.as_u32());
        debug_struct = debug_struct.field(
            "path",
            &::salsa::debug::helper::SalsaDebug::<
                AbsolutePath,
                <VfsJar as salsa::jar::Jar<'_>>::DynDb,
            >::salsa_debug(
                #[allow(clippy::needless_borrow)]
                &self.path(_db),
                _db,
                _include_all_fields,
            ),
        );
        if _include_all_fields {
            debug_struct = debug_struct.field(
                "content",
                &::salsa::debug::helper::SalsaDebug::<
                    FileContent,
                    <VfsJar as salsa::jar::Jar<'_>>::DynDb,
                >::salsa_debug(
                    #[allow(clippy::needless_borrow)]
                    &self.content(_db),
                    _db,
                    _include_all_fields,
                ),
            );
        }
        debug_struct.finish()
    }
}
impl<DB> salsa::salsa_struct::SalsaStructInDb<DB> for File
where
    DB: ?Sized + salsa::DbWithJar<VfsJar>,
{
    fn register_dependent_fn(_db: &DB, _index: salsa::routes::IngredientIndex) {}
}

#[salsa::tracked(jar = VfsJar )]
pub(crate) fn package_manifest_file(db: &dyn VfsDb, package_path: PackagePath) -> VfsResult<File> {
    Ok(db.file_from_absolute_path(&package_manifest_path(db, package_path)?))
}

#[salsa::tracked(jar = VfsJar )]
pub(crate) fn module_file(db: &dyn VfsDb, entity_path: EntityPath) -> VfsResult<File> {
    let abs_path = module_path(db, entity_path).as_ref()?;
    Ok(db.file_from_absolute_path(abs_path))
}
