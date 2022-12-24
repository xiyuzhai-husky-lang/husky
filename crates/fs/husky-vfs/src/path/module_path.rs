mod ancestry;

pub use ancestry::*;
use salsa::DebugWithDb;
use with_db::{PartialOrdWithDb, WithDb};

use super::*;

/// one module path is large if it contains more files
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ModulePath(salsa::Id);

impl ModulePath {
    pub fn starts_with(self, db: &dyn VfsDb, parent: ModulePath) -> bool {
        self.module_ancestry(db).contains(parent)
    }

    pub fn module_ancestry(self, db: &dyn VfsDb) -> &ModuleAncestry {
        module_ancestry(db, self)
    }
}

impl PartialOrdWithDb<dyn VfsDb + '_> for ModulePath {
    fn partial_cmp_with_db(&self, db: &dyn VfsDb, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            return Some(std::cmp::Ordering::Equal);
        }
        if self.starts_with(db, *other) {
            return Some(std::cmp::Ordering::Less);
        }
        if other.starts_with(db, *self) {
            return Some(std::cmp::Ordering::Greater);
        }
        None
    }
}

impl<Db: VfsDb> PartialOrdWithDb<Db> for ModulePath {
    fn partial_cmp_with_db(&self, db: &Db, other: &Self) -> Option<std::cmp::Ordering> {
        self.partial_cmp_with_db(db as &dyn VfsDb, other)
    }
}

#[test]
fn module_path_partial_ord_works() {
    let db = DB::default();
    let path_menu = db.dev_path_menu().unwrap();

    assert!(path_menu.core().with_db(&db) > (path_menu.core_num()).with_db(&db));
    assert!(!(path_menu.core().with_db(&db) == (path_menu.core_num()).with_db(&db)));
    assert!(!(path_menu.core().with_db(&db) < (path_menu.core_num()).with_db(&db)));
    assert!(!(path_menu.core().with_db(&db) <= (path_menu.core_num()).with_db(&db)));
    assert!(path_menu.core().with_db(&db) >= (path_menu.core_num()).with_db(&db));
    assert!(path_menu.core().with_db(&db) != (path_menu.core_num()).with_db(&db));

    assert!(!(path_menu.core_prelude().with_db(&db) > path_menu.core_num().with_db(&db)));
    assert!(!(path_menu.core_prelude().with_db(&db) == path_menu.core_num().with_db(&db)));
    assert!(!(path_menu.core_prelude().with_db(&db) < path_menu.core_num().with_db(&db)));
    assert!(!(path_menu.core_prelude().with_db(&db) <= path_menu.core_num().with_db(&db)));
    assert!(!(path_menu.core_prelude().with_db(&db) >= path_menu.core_num().with_db(&db)));
    assert!(path_menu.core_prelude().with_db(&db) != path_menu.core_num().with_db(&db));

    assert_ne!(
        path_menu.std().with_db(&db),
        path_menu.core_ops().with_db(&db),
    )
}

#[doc = r" Internal struct used for interned item"]
#[derive(Eq, PartialEq, Hash, Clone)]
pub struct __ModulePathData {
    data: ModulePathData,
}
impl salsa::storage::IngredientsFor for ModulePath {
    type Jar = VfsJar;
    type Ingredients = salsa::interned::InternedIngredient<ModulePath, __ModulePathData>;
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
        salsa::interned::InternedIngredient::new(index, "ModulePath")
    }
}
impl salsa::AsId for ModulePath {
    fn as_id(self) -> salsa::Id {
        self.0
    }
    fn from_id(id: salsa::Id) -> Self {
        ModulePath(id)
    }
}
impl ModulePath {
    pub fn data(self, db: &<VfsJar as salsa::jar::Jar<'_>>::DynDb) -> ModulePathData {
        let (jar, runtime) = <_ as salsa::storage::HasJar<VfsJar>>::jar(db);
        let ingredients =
            <VfsJar as salsa::storage::HasIngredientsFor<ModulePath>>::ingredient(jar);
        std::clone::Clone::clone(&ingredients.data(runtime, self).data)
    }

    pub fn new(db: &<VfsJar as salsa::jar::Jar<'_>>::DynDb, data: ModulePathData) -> Self {
        let (jar, runtime) = <_ as salsa::storage::HasJar<VfsJar>>::jar(db);
        let ingredients =
            <VfsJar as salsa::storage::HasIngredientsFor<ModulePath>>::ingredient(jar);
        ingredients.intern(runtime, __ModulePathData { data })
    }

    pub fn crate_path(self, db: &dyn VfsDb) -> CratePath {
        match self.data(db) {
            ModulePathData::Root(crate_path) => crate_path,
            ModulePathData::Child { parent, ident } => parent.crate_path(db),
        }
    }

    pub fn new_root(db: &dyn VfsDb, crate_path: CratePath) -> Self {
        Self::new(db, ModulePathData::Root(crate_path))
    }

    pub fn new_child(db: &dyn VfsDb, parent: ModulePath, ident: Identifier) -> Self {
        Self::new(db, ModulePathData::Child { parent, ident })
    }

    pub fn toolchain(self, db: &dyn VfsDb) -> Toolchain {
        self.crate_path(db).toolchain(db)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModulePathData {
    Root(CratePath),
    Child {
        parent: ModulePath,
        ident: Identifier,
    },
}

impl ModulePathData {
    fn display(self, db: &dyn VfsDb, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModulePathData::Root(crate_path) => f.write_str("crate"),
            ModulePathData::Child { parent, ident } => {
                parent.data(db).display(db, f)?;
                f.write_str("::");
                f.write_str(ident.data(db))
            }
        }
    }
}

impl<Db: VfsDb> salsa::DebugWithDb<Db> for ModulePathData {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        self.fmt(f, db as &dyn VfsDb, include_all_fields)
    }
}

impl<DB> salsa::salsa_struct::SalsaStructInDb<DB> for ModulePath
where
    DB: ?Sized + salsa::DbWithJar<VfsJar>,
{
    fn register_dependent_fn(_db: &DB, _index: salsa::routes::IngredientIndex) {}
}
impl ::salsa::DebugWithDb<<VfsJar as salsa::jar::Jar<'_>>::DynDb> for ModulePath {
    fn fmt(
        &self,
        f: &mut ::std::fmt::Formatter<'_>,
        db: &<VfsJar as salsa::jar::Jar<'_>>::DynDb,
        include_all_fields: bool,
    ) -> ::std::fmt::Result {
        #[allow(unused_imports)]
        use ::salsa::debug::helper::Fallback;
        if include_all_fields {
            f.debug_struct("ModulePath")
                .field(
                    "[display]",
                    &::salsa::debug::helper::SalsaDebug::<
                        ModulePathData,
                        <VfsJar as salsa::jar::Jar<'_>>::DynDb,
                    >::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        &self.data(db),
                        db,
                        include_all_fields,
                    ),
                )
                .field(
                    "[crate]",
                    &self.crate_path(db).debug_with(db, include_all_fields),
                )
                .finish()
        } else {
            self.show(f, db)
        }
    }
}

impl ModulePath {
    pub fn show(
        &self,
        f: &mut ::std::fmt::Formatter<'_>,
        db: &<VfsJar as salsa::jar::Jar<'_>>::DynDb,
    ) -> ::std::fmt::Result {
        f.write_str("`")?;
        self.show_aux(f, db)?;
        f.write_str("`")
    }
    pub fn show_aux(
        &self,
        f: &mut ::std::fmt::Formatter<'_>,
        db: &<VfsJar as salsa::jar::Jar<'_>>::DynDb,
    ) -> ::std::fmt::Result {
        match self.data(db) {
            ModulePathData::Root(crate_path) => match crate_path.package_ident(db) {
                Ok(ident) => f.write_str(ident.data(db)),
                Err(e) => e.fmt(f, db, true),
            },
            ModulePathData::Child { parent, ident } => {
                parent.show_aux(f, db)?;
                f.write_str("::")?;
                f.write_str(ident.data(db))
            }
        }
    }
}

#[test]
fn module_path_debug_with_db_works() {
    fn t(db: &DB, module_path: ModulePath, include_all_fields: bool, expect: &str) {
        assert_eq!(
            format!("{:?}", module_path.debug_with(db, include_all_fields)),
            expect
        )
    }
    let db = DB::default();
    let path_menu = db.dev_path_menu().unwrap();
    t(&db, path_menu.core_num(), false, "`core::num`");
    t(&db, path_menu.core(), false, "`core`");
    t(&db, path_menu.std(), false, "`std`");
    expect_test::expect![[r#"
        `core`
    "#]]
    .assert_debug_eq(&path_menu.core().debug(&db));
    expect_test::expect![[r#"
        `core::num`
    "#]]
    .assert_debug_eq(&path_menu.core_num().debug(&db));
    expect_test::expect![[r#"
        `std`
    "#]]
    .assert_debug_eq(&path_menu.std().debug(&db));
}

impl salsa::DebugWithDb<dyn VfsDb + '_> for ModulePathData {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn VfsDb,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        f.write_str("\"")?;
        self.display(db, f)?;
        f.write_str("\"")
    }
}

impl<Db: VfsDb> salsa::DebugWithDb<Db> for ModulePath {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        self.fmt(f, db as &dyn VfsDb, include_all_fields)
    }
}
