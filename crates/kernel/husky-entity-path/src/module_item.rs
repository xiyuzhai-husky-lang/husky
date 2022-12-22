use salsa::DebugWithDb;

use crate::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub struct ModuleItemPath(salsa::Id);

#[doc = r" Internal struct used for interned item"]
#[derive(Eq, PartialEq, Hash, Clone)]
pub struct __ModuleItemPathData {
    module: ModulePath,
    ident: Identifier,
}
impl salsa::storage::IngredientsFor for ModuleItemPath {
    type Jar = EntityPathJar;
    type Ingredients = salsa::interned::InternedIngredient<ModuleItemPath, __ModuleItemPathData>;
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
        salsa::interned::InternedIngredient::new(index, "ModuleItemPath")
    }
}
impl salsa::AsId for ModuleItemPath {
    fn as_id(self) -> salsa::Id {
        self.0
    }
    fn from_id(id: salsa::Id) -> Self {
        ModuleItemPath(id)
    }
}
impl ModuleItemPath {
    fn module(self, db: &<EntityPathJar as salsa::jar::Jar<'_>>::DynDb) -> ModulePath {
        let (jar, runtime) = <_ as salsa::storage::HasJar<EntityPathJar>>::jar(db);
        let ingredients =
            <EntityPathJar as salsa::storage::HasIngredientsFor<ModuleItemPath>>::ingredient(jar);
        std::clone::Clone::clone(&ingredients.data(runtime, self).module)
    }
    fn ident(self, db: &<EntityPathJar as salsa::jar::Jar<'_>>::DynDb) -> Identifier {
        let (jar, runtime) = <_ as salsa::storage::HasJar<EntityPathJar>>::jar(db);
        let ingredients =
            <EntityPathJar as salsa::storage::HasIngredientsFor<ModuleItemPath>>::ingredient(jar);
        std::clone::Clone::clone(&ingredients.data(runtime, self).ident)
    }
    pub fn new(
        db: &<EntityPathJar as salsa::jar::Jar<'_>>::DynDb,
        module: ModulePath,
        ident: Identifier,
    ) -> Self {
        let (jar, runtime) = <_ as salsa::storage::HasJar<EntityPathJar>>::jar(db);
        let ingredients =
            <EntityPathJar as salsa::storage::HasIngredientsFor<ModuleItemPath>>::ingredient(jar);
        ingredients.intern(runtime, __ModuleItemPathData { module, ident })
    }
}
impl<DB> salsa::salsa_struct::SalsaStructInDb<DB> for ModuleItemPath
where
    DB: ?Sized + salsa::DbWithJar<EntityPathJar>,
{
    fn register_dependent_fn(_db: &DB, _index: salsa::routes::IngredientIndex) {}
}
impl ::salsa::DebugWithDb<<EntityPathJar as salsa::jar::Jar<'_>>::DynDb> for ModuleItemPath {
    fn fmt(
        &self,
        f: &mut ::std::fmt::Formatter<'_>,
        db: &<EntityPathJar as salsa::jar::Jar<'_>>::DynDb,
        include_all_fields: bool,
    ) -> ::std::fmt::Result {
        if include_all_fields {
            f.debug_struct("ModuleItemPath")
                .field(
                    "module",
                    &self
                        .module(db)
                        .debug_with(db as &dyn VfsDb, include_all_fields),
                )
                .field("ident", &self.ident(db).debug_with(db, include_all_fields))
                .finish()
        } else {
            f.write_str("`")?;
            self.module(db).show_aux(f, db)?;
            f.write_str("::")?;
            f.write_str(self.ident(db).data(db))?;
            f.write_str("`")
        }
    }
}

impl<Db: EntityPathDb> salsa::DebugWithDb<Db> for ModuleItemPath {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        self.fmt(f, db as &dyn EntityPathDb, include_all_fields)
    }
}

#[test]
fn module_item_path_debug_with_db_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain();
    let entity_path_menu = db.entity_path_menu(toolchain).unwrap();
    expect_test::expect![[r#"
        `core::num::b32`
    "#]]
    .assert_debug_eq(&entity_path_menu.b32().debug(&db));
}
