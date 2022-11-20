use timed_salsa::{input::InputIngredient, input_field::InputFieldIngredient, Durability};

use crate::*;

#[derive(Debug, Clone, Copy)]
pub enum HuskyFileClass {
    Library,
    Publish,
    User,
}

impl HuskyFileClass {
    fn durability(self) -> Durability {
        match self {
            HuskyFileClass::Library => Durability::HIGH,
            HuskyFileClass::Publish => Durability::HIGH,
            HuskyFileClass::User => Durability::LOW,
        }
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub struct HuskyFileId(timed_salsa::Id);

impl HuskyFileId {
    pub fn new(
        db: &<crate::Jar as timed_salsa::jar::Jar<'_>>::DynDb,
        path: PathBufItd,
        source_class: HuskyFileClass,
        content: String,
    ) -> Self {
        let (jar, runtime) = <_ as timed_salsa::storage::HasJar<crate::Jar>>::jar(db);
        let ingredients =
            <crate::Jar as timed_salsa::storage::HasIngredientsFor<HuskyFileId>>::ingredient(jar);
        let id = ingredients.3.new_input(runtime);
        ingredients.0.store_new(runtime, id, path, Durability::HIGH);
        ingredients
            .1
            .store_new(runtime, id, source_class, Durability::HIGH);
        ingredients
            .2
            .store_new(runtime, id, content, source_class.durability());
        id
    }

    fn path<'db>(self, __db: &'db <crate::Jar as timed_salsa::jar::Jar<'_>>::DynDb) -> PathBufItd {
        let (__jar, __runtime) = <_ as timed_salsa::storage::HasJar<crate::Jar>>::jar(__db);
        let __ingredients =
            <crate::Jar as timed_salsa::storage::HasIngredientsFor<HuskyFileId>>::ingredient(__jar);
        __ingredients.0.fetch(__runtime, self).clone()
    }

    fn source_class<'db>(
        self,
        __db: &'db <crate::Jar as timed_salsa::jar::Jar<'_>>::DynDb,
    ) -> HuskyFileClass {
        let (__jar, __runtime) = <_ as timed_salsa::storage::HasJar<crate::Jar>>::jar(__db);
        let __ingredients =
            <crate::Jar as timed_salsa::storage::HasIngredientsFor<HuskyFileId>>::ingredient(__jar);
        __ingredients.1.fetch(__runtime, self).clone()
    }

    pub fn content<'db>(
        self,
        __db: &'db <crate::Jar as timed_salsa::jar::Jar<'_>>::DynDb,
    ) -> &'db String {
        let (__jar, __runtime) = <_ as timed_salsa::storage::HasJar<crate::Jar>>::jar(__db);
        let __ingredients =
            <crate::Jar as timed_salsa::storage::HasIngredientsFor<HuskyFileId>>::ingredient(__jar);
        __ingredients.2.fetch(__runtime, self)
    }

    fn set_kind<'db>(
        self,
        __db: &'db mut <crate::Jar as timed_salsa::jar::Jar<'_>>::DynDb,
    ) -> timed_salsa::setter::Setter<'db, HuskyFileId, HuskyFileClass> {
        let (__jar, __runtime) = <_ as timed_salsa::storage::HasJar<crate::Jar>>::jar_mut(__db);
        let __ingredients =
            <crate::Jar as timed_salsa::storage::HasIngredientsFor<HuskyFileId>>::ingredient_mut(
                __jar,
            );
        timed_salsa::setter::Setter::new(__runtime, self, &mut __ingredients.1)
    }

    pub(crate) fn set_content<'db>(
        self,
        db: &'db mut <crate::Jar as timed_salsa::jar::Jar<'_>>::DynDb,
    ) -> timed_salsa::setter::Setter<'db, HuskyFileId, String> {
        match self.source_class(db) {
            HuskyFileClass::Library => todo!(),
            HuskyFileClass::Publish => todo!(),
            HuskyFileClass::User => (),
        }
        let (__jar, __runtime) = <_ as timed_salsa::storage::HasJar<crate::Jar>>::jar_mut(db);
        let __ingredients =
            <crate::Jar as timed_salsa::storage::HasIngredientsFor<HuskyFileId>>::ingredient_mut(
                __jar,
            );
        timed_salsa::setter::Setter::new(__runtime, self, &mut __ingredients.2)
    }
}

impl timed_salsa::storage::IngredientsFor for HuskyFileId {
    type Jar = crate::Jar;
    type Ingredients = (
        InputFieldIngredient<HuskyFileId, PathBufItd>,
        InputFieldIngredient<HuskyFileId, HuskyFileClass>,
        InputFieldIngredient<HuskyFileId, String>,
        InputIngredient<HuskyFileId>,
    );
    fn create_ingredients<DB>(routes: &mut timed_salsa::routes::Routes<DB>) -> Self::Ingredients
    where
        DB: timed_salsa::DbWithJar<Self::Jar> + timed_salsa::storage::JarFromJars<Self::Jar>,
    {
        (
            {
                let index = routes.push(
                    |jars| {
                        let jar =
                            <DB as timed_salsa::storage::JarFromJars<Self::Jar>>::jar_from_jars(
                                jars,
                            );
                        let ingredients =
                            <_ as timed_salsa::storage::HasIngredientsFor<Self>>::ingredient(jar);
                        &ingredients.0
                    },
                    |jars| {
                        let jar =
                            <DB as timed_salsa::storage::JarFromJars<Self::Jar>>::jar_from_jars_mut(
                                jars,
                            );
                        let ingredients =
                            <_ as timed_salsa::storage::HasIngredientsFor<Self>>::ingredient_mut(
                                jar,
                            );
                        &mut ingredients.0
                    },
                );
                InputFieldIngredient::new(index, "path")
            },
            {
                let index = routes.push(
                    |jars| {
                        let jar =
                            <DB as timed_salsa::storage::JarFromJars<Self::Jar>>::jar_from_jars(
                                jars,
                            );
                        let ingredients =
                            <_ as timed_salsa::storage::HasIngredientsFor<Self>>::ingredient(jar);
                        &ingredients.1
                    },
                    |jars| {
                        let jar =
                            <DB as timed_salsa::storage::JarFromJars<Self::Jar>>::jar_from_jars_mut(
                                jars,
                            );
                        let ingredients =
                            <_ as timed_salsa::storage::HasIngredientsFor<Self>>::ingredient_mut(
                                jar,
                            );
                        &mut ingredients.1
                    },
                );
                InputFieldIngredient::new(index, "kind")
            },
            {
                let index = routes.push(
                    |jars| {
                        let jar =
                            <DB as timed_salsa::storage::JarFromJars<Self::Jar>>::jar_from_jars(
                                jars,
                            );
                        let ingredients =
                            <_ as timed_salsa::storage::HasIngredientsFor<Self>>::ingredient(jar);
                        &ingredients.2
                    },
                    |jars| {
                        let jar =
                            <DB as timed_salsa::storage::JarFromJars<Self::Jar>>::jar_from_jars_mut(
                                jars,
                            );
                        let ingredients =
                            <_ as timed_salsa::storage::HasIngredientsFor<Self>>::ingredient_mut(
                                jar,
                            );
                        &mut ingredients.2
                    },
                );
                InputFieldIngredient::new(index, "content")
            },
            {
                let index = routes.push(
                    |jars| {
                        let jar =
                            <DB as timed_salsa::storage::JarFromJars<Self::Jar>>::jar_from_jars(
                                jars,
                            );
                        let ingredients =
                            <_ as timed_salsa::storage::HasIngredientsFor<Self>>::ingredient(jar);
                        &ingredients.3
                    },
                    |jars| {
                        let jar =
                            <DB as timed_salsa::storage::JarFromJars<Self::Jar>>::jar_from_jars_mut(
                                jars,
                            );
                        let ingredients =
                            <_ as timed_salsa::storage::HasIngredientsFor<Self>>::ingredient_mut(
                                jar,
                            );
                        &mut ingredients.3
                    },
                );
                InputIngredient::new(index, "HuskyFile")
            },
        )
    }
}
impl timed_salsa::AsId for HuskyFileId {
    fn as_id(self) -> timed_salsa::Id {
        self.0
    }
    fn from_id(id: timed_salsa::Id) -> Self {
        HuskyFileId(id)
    }
}
impl ::timed_salsa::DebugWithDb<<crate::Jar as timed_salsa::jar::Jar<'_>>::DynDb> for HuskyFileId {
    fn fmt(
        &self,
        f: &mut ::std::fmt::Formatter<'_>,
        _db: &<crate::Jar as timed_salsa::jar::Jar<'_>>::DynDb,
        _include_all_fields: bool,
    ) -> ::std::fmt::Result {
        #[allow(unused_imports)]
        use ::timed_salsa::debug::helper::Fallback;
        let mut debug_struct = &mut f.debug_struct("HuskyFile");
        debug_struct = debug_struct.field("[salsa id]", &self.0.as_u32());
        debug_struct = debug_struct.field(
            "path",
            &::timed_salsa::debug::helper::SalsaDebug::<
                PathBufItd,
                <crate::Jar as timed_salsa::jar::Jar<'_>>::DynDb,
            >::salsa_debug(
                #[allow(clippy::needless_borrow)]
                &self.path(_db),
                _db,
                _include_all_fields,
            ),
        );
        if _include_all_fields {
            debug_struct = debug_struct.field(
                "kind",
                &::timed_salsa::debug::helper::SalsaDebug::<
                    HuskyFileClass,
                    <crate::Jar as timed_salsa::jar::Jar<'_>>::DynDb,
                >::salsa_debug(
                    #[allow(clippy::needless_borrow)]
                    &self.source_class(_db),
                    _db,
                    _include_all_fields,
                ),
            );
        }
        if _include_all_fields {
            debug_struct = debug_struct.field(
                "content",
                &::timed_salsa::debug::helper::SalsaDebug::<
                    String,
                    <crate::Jar as timed_salsa::jar::Jar<'_>>::DynDb,
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
impl<DB> timed_salsa::salsa_struct::SalsaStructInDb<DB> for HuskyFileId
where
    DB: ?Sized + timed_salsa::DbWithJar<crate::Jar>,
{
    fn register_dependent_fn(_db: &DB, _index: timed_salsa::routes::IngredientIndex) {}
}
