mod ancestry;

use super::*;
pub use ancestry::*;
use salsa::{DbWithJar, DebugWithDb, DisplayWithDb};
use with_db::{PartialOrdWithDb, WithDb};

#[salsa::interned(jar = VfsJar, override_debug)]
pub struct ModulePath {
    pub data: ModulePathData,
}

impl ModulePath {
    pub fn starts_with(self, db: &dyn VfsDb, parent: ModulePath) -> bool {
        self.module_ancestry(db).contains(parent)
    }

    pub fn module_ancestry(self, db: &dyn VfsDb) -> &ModuleAncestry {
        module_ancestry(db, self)
    }

    pub fn crate_path(self, db: &dyn VfsDb) -> CratePath {
        self.module_ancestry(db).crate_path()
    }

    pub fn package_path(self, db: &dyn VfsDb) -> PackagePath {
        self.crate_path(db).package_path(db)
    }

    pub fn new_root(db: &dyn VfsDb, crate_path: CratePath) -> Self {
        Self::new(db, ModulePathData::Root(crate_path))
    }

    pub fn new_child(db: &dyn VfsDb, parent: ModulePath, ident: Ident) -> Self {
        Self::new(db, ModulePathData::Child { parent, ident })
    }

    pub fn toolchain(self, db: &dyn VfsDb) -> Toolchain {
        self.crate_path(db).toolchain(db)
    }

    pub fn ident(self, db: &dyn VfsDb) -> VfsResult<Ident> {
        match self.data(db) {
            ModulePathData::Root(crate_path) => crate_path.package_ident(db),
            ModulePathData::Child { parent: _, ident } => Ok(ident),
        }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModulePathData {
    Root(CratePath),
    Child { parent: ModulePath, ident: Ident },
}

impl ModulePathData {
    fn display(self, db: &dyn VfsDb, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModulePathData::Root(_crate_path) => f.write_str("crate"),
            ModulePathData::Child { parent, ident } => {
                parent.data(db).display(db, f)?;
                f.write_str("::");
                f.write_str(ident.data(db))
            }
        }
    }
}

impl ModulePath {
    pub fn to_string_with_db(&self, db: &dyn VfsDb) -> String {
        self.display(db).to_string()
    }

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
                Err(e) => e.fmt(f, db, salsa::DebugFormatLevel::root()),
            },
            ModulePathData::Child { parent, ident } => {
                parent.show_aux(f, db)?;
                f.write_str("::")?;
                f.write_str(ident.data(db))
            }
        }
    }
}

impl<Db> salsa::DisplayWithDb<Db> for ModulePath
where
    Db: VfsDb + ?Sized,
{
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        _level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<VfsJar>>::as_jar_db(db);
        self.show_aux(f, db)
    }
}

#[test]
fn module_path_debug_with_db_works() {
    fn t(db: &DB, module_path: ModulePath, level: salsa::DebugFormatLevel, expect: &str) {
        assert_eq!(
            format!("{:?}", module_path.debug_with(db, level.next())),
            expect
        )
    }
    let db = DB::default();
    let path_menu = db.dev_path_menu().unwrap();
    t(
        &db,
        path_menu.core_num(),
        salsa::DebugFormatLevel::root(),
        "`core::num`",
    );
    t(
        &db,
        path_menu.core(),
        salsa::DebugFormatLevel::root(),
        "`core`",
    );
    t(
        &db,
        path_menu.std(),
        salsa::DebugFormatLevel::root(),
        "`std`",
    );
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

impl<Db: VfsDb + ?Sized> salsa::DebugWithDb<Db> for ModulePath {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        _level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        #[allow(unused_imports)]
        use ::salsa::debug::helper::Fallback;
        let db = <Db as DbWithJar<VfsJar>>::as_jar_db(db);
        self.show(f, db)
    }
}
