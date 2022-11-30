use husky_source_path::SourcePath;
use salsa::{input::InputIngredient, input_field::InputFieldIngredient, Durability};

use crate::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub struct SourceFile(salsa::Id);

impl SourceFile {
    pub fn new(
        db: &<crate::VfsJar as salsa::jar::Jar<'_>>::DynDb,
        path: SourcePath,
        content: String,
    ) -> Self {
        let (jar, runtime) = <_ as salsa::storage::HasJar<VfsJar>>::jar(db);
        let ingredients =
            <VfsJar as salsa::storage::HasIngredientsFor<SourceFile>>::ingredient(jar);
        let id = ingredients.2.new_input(runtime);
        ingredients.0.store_new(runtime, id, path, Durability::HIGH);
        ingredients
            .1
            .store_new(runtime, id, content, db.source_durability(path));
        id
    }

    pub fn path<'db>(self, __db: &'db <VfsJar as salsa::jar::Jar<'_>>::DynDb) -> SourcePath {
        let (__jar, __runtime) = <_ as salsa::storage::HasJar<VfsJar>>::jar(__db);
        let __ingredients =
            <VfsJar as salsa::storage::HasIngredientsFor<SourceFile>>::ingredient(__jar);
        __ingredients.0.fetch(__runtime, self).clone()
    }

    pub fn content<'db>(self, __db: &'db <VfsJar as salsa::jar::Jar<'_>>::DynDb) -> &'db String {
        let (__jar, __runtime) = <_ as salsa::storage::HasJar<VfsJar>>::jar(__db);
        let __ingredients =
            <VfsJar as salsa::storage::HasIngredientsFor<SourceFile>>::ingredient(__jar);
        __ingredients.1.fetch(__runtime, self)
    }

    pub(crate) fn set_content<'db>(
        self,
        db: &'db mut <VfsJar as salsa::jar::Jar<'_>>::DynDb,
    ) -> salsa::setter::Setter<'db, SourceFile, String> {
        let (__jar, __runtime) = <_ as salsa::storage::HasJar<VfsJar>>::jar_mut(db);
        let __ingredients =
            <VfsJar as salsa::storage::HasIngredientsFor<SourceFile>>::ingredient_mut(__jar);
        salsa::setter::Setter::new(__runtime, self, &mut __ingredients.1)
    }
}

impl salsa::storage::IngredientsFor for SourceFile {
    type Jar = VfsJar;
    type Ingredients = (
        InputFieldIngredient<SourceFile, SourcePath>,
        InputFieldIngredient<SourceFile, String>,
        InputIngredient<SourceFile>,
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
impl salsa::AsId for SourceFile {
    fn as_id(self) -> salsa::Id {
        self.0
    }
    fn from_id(id: salsa::Id) -> Self {
        SourceFile(id)
    }
}
impl ::salsa::DebugWithDb<<VfsJar as salsa::jar::Jar<'_>>::DynDb> for SourceFile {
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
        debug_struct =
            debug_struct.field(
                "path",
                &::salsa::debug::helper::SalsaDebug::<
                    SourcePath,
                    <VfsJar as salsa::jar::Jar<'_>>::DynDb,
                >::salsa_debug(
                    #[allow(clippy::needless_borrow)]
                    &self.path(_db),
                    _db,
                    _include_all_fields,
                ),
            );
        if _include_all_fields {
            debug_struct =
                debug_struct.field(
                    "content",
                    &::salsa::debug::helper::SalsaDebug::<
                        String,
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
impl<DB> salsa::salsa_struct::SalsaStructInDb<DB> for SourceFile
where
    DB: ?Sized + salsa::DbWithJar<VfsJar>,
{
    fn register_dependent_fn(_db: &DB, _index: salsa::routes::IngredientIndex) {}
}
