use crate::*;
use salsa::{input::InputIngredient, input_field::InputFieldIngredient, Db, Durability};

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub struct File(salsa::Id);

#[derive(Debug, PartialEq, Eq)]
pub enum FileContent {
    NotExists,
    OnDisk(String),
    LiveDoc(String),
    Directory(Vec<VirtualPath>),
    Err(VfsError),
}

impl File {
    pub fn text(self, db: &::salsa::Db) -> VfsResult<Option<&str>> {
        self.content(db).text(self.path(db).data(db))
    }
}

impl FileContent {
    pub(crate) fn text(&self, _path: &Path) -> VfsResult<Option<&str>> {
        match self {
            FileContent::NotExists => Ok(None),
            FileContent::OnDisk(text) | FileContent::LiveDoc(text) => Ok(Some(text)),
            FileContent::Directory(_) => todo!(),
            FileContent::Err(_) => todo!(),
        }
    }
}

impl File {
    pub(crate) fn new(
        db: &::salsa::Db,
        path: VirtualPath,
        content: FileContent,
        durability: Durability,
    ) -> Self {
        let (jar, runtime) = db.jar::<VfsJar>();
        let ingredients = <VfsJar as salsa::storage::HasIngredientsFor<File>>::ingredient(jar);
        let id = ingredients.2.new_input(runtime);
        ingredients.0.store_new(runtime, id, path, Durability::HIGH);
        ingredients.1.store_new(runtime, id, content, durability);
        id
    }

    pub(crate) fn path(self, __db: &::salsa::Db) -> VirtualPath {
        let (__jar, __runtime) = __db.jar::<VfsJar>();
        let __ingredients = <VfsJar as salsa::storage::HasIngredientsFor<File>>::ingredient(__jar);
        __ingredients.0.fetch(__runtime, self).clone()
    }

    pub(crate) fn content<'db>(self, __db: &'db Db) -> &'db FileContent {
        let (__jar, __runtime) = __db.jar::<VfsJar>();
        let __ingredients = <VfsJar as salsa::storage::HasIngredientsFor<File>>::ingredient(__jar);
        __ingredients.1.fetch(__runtime, self)
    }

    pub(crate) fn set_content<'db>(
        self,
        db: &'db mut Db,
    ) -> salsa::setter::Setter<'db, File, FileContent> {
        let (__jar, __runtime) = db.jar_mut();
        let __ingredients =
            <VfsJar as salsa::storage::HasIngredientsFor<File>>::ingredient_mut(__jar);
        salsa::setter::Setter::new(__runtime, self, &mut __ingredients.1)
    }
}

impl salsa::storage::IngredientsFor for File {
    type Jar = VfsJar;
    type Ingredients = (
        InputFieldIngredient<File, VirtualPath>,
        InputFieldIngredient<File, FileContent>,
        InputIngredient<File>,
    );
    fn create_ingredients(routes: &mut salsa::routes::Routes) -> Self::Ingredients {
        (
            {
                let index = routes.push(
                    |jars| {
                        let jar = jars.jar::<Self::Jar>();
                        let ingredients =
                            <_ as salsa::storage::HasIngredientsFor<Self>>::ingredient(jar);
                        &ingredients.0
                    },
                    |jars| {
                        let jar = jars.jar_mut::<Self::Jar>();
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
                        let jar = jars.jar::<Self::Jar>();
                        let ingredients =
                            <_ as salsa::storage::HasIngredientsFor<Self>>::ingredient(jar);
                        &ingredients.1
                    },
                    |jars| {
                        let jar = jars.jar_mut::<Self::Jar>();
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
                        let jar = jars.jar::<Self::Jar>();
                        let ingredients =
                            <_ as salsa::storage::HasIngredientsFor<Self>>::ingredient(jar);
                        &ingredients.2
                    },
                    |jars| {
                        let jar = jars.jar_mut::<Self::Jar>();
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
impl ::salsa::DebugWithDb for File {
    fn debug_with_db_fmt(
        &self,
        f: &mut ::std::fmt::Formatter<'_>,
        _db: &::salsa::Db,
    ) -> ::std::fmt::Result {
        #[allow(unused_imports)]
        use salsa::debug::helper::Fallback;
        let mut debug_struct = &mut f.debug_struct("HuskyFile");
        debug_struct = debug_struct.field(
            "path",
            &::salsa::debug::helper::SalsaDebug::<VirtualPath>::salsa_debug(
                #[allow(clippy::needless_borrow)]
                &self.path(_db),
                _db,
            ),
        );
        debug_struct = debug_struct.field(
            "content",
            &::salsa::debug::helper::SalsaDebug::<FileContent>::salsa_debug(
                #[allow(clippy::needless_borrow)]
                &self.content(_db),
                _db,
            ),
        );
        debug_struct.finish()
    }
}

impl salsa::salsa_struct::SalsaStructInDb for File {
    fn register_dependent_fn(_db: &::salsa::Db, _index: salsa::routes::IngredientIndex) {}
}
