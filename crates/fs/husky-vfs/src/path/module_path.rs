pub mod ancestry;
pub mod relative_path;

use super::*;
pub use ancestry::*;
use salsa::DisplayWithDb;
use with_db::PartialOrdWithDb;
#[cfg(test)]
use with_db::WithDb;

#[salsa::interned(jar = VfsJar, override_debug, constructor = new_inner)]
pub struct ModulePath {
    pub data: ModulePathData,
}

impl ModulePath {
    pub fn new(db: &::salsa::Db, data: ModulePathData) -> VfsResult<Self> {
        let slf = Self::new_inner(db, data);
        let diff_path = module_virtual_path(db, slf)?;
        db.file_from_virtual_path(diff_path)?
            .text(db)?
            .ok_or(VfsError::FileNotExists(diff_path))?;
        Ok(slf)
    }
}

/// wrapper type that guarantees that the inner field is a submodule
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[salsa::as_id(jar = VfsJar)]
#[salsa::deref_id]
pub struct SubmodulePath(ModulePath);

impl SubmodulePath {
    /// returns the natural casting
    /// not the parent
    pub fn inner(self) -> ModulePath {
        self.0
    }

    pub fn parent(self, db: &::salsa::Db) -> ModulePath {
        self.0.parent(db).unwrap()
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        self.parent(db)
    }

    pub fn ident(self, db: &::salsa::Db) -> Ident {
        self.0.ident(db)
    }
}

impl From<SubmodulePath> for ModulePath {
    fn from(path: SubmodulePath) -> Self {
        path.0
    }
}

impl ModulePath {
    pub fn starts_with(self, db: &::salsa::Db, parent: ModulePath) -> bool {
        self.module_ancestry(db).contains(parent)
    }

    pub fn module_ancestry(self, db: &::salsa::Db) -> &ModuleAncestry {
        module_ancestry(db, self)
    }

    pub fn crate_path(self, db: &::salsa::Db) -> CratePath {
        self.module_ancestry(db).crate_path()
    }

    pub fn package_path(self, db: &::salsa::Db) -> PackagePath {
        self.crate_path(db).package_path(db)
    }

    pub fn parent(self, db: &::salsa::Db) -> Option<Self> {
        match self.data(db) {
            ModulePathData::Root(_) => None,
            ModulePathData::Child { parent, .. } => Some(parent),
        }
    }

    /// use CratePath::root_module_path instead in other crates
    pub(crate) fn new_root(db: &::salsa::Db, crate_path: CratePath) -> VfsResult<Self> {
        Self::new(db, ModulePathData::Root(crate_path))
    }

    pub fn new_child(
        db: &::salsa::Db,
        parent: ModulePath,
        ident: Ident,
    ) -> VfsResult<SubmodulePath> {
        Ok(SubmodulePath(Self::new(
            db,
            ModulePathData::Child { parent, ident },
        )?))
    }

    pub fn toolchain(self, db: &::salsa::Db) -> Toolchain {
        self.crate_path(db).toolchain(db)
    }

    #[inline(always)]
    pub fn virtual_path(self, db: &::salsa::Db) -> VirtualPath {
        module_virtual_path(db, self).expect("guaranteed")
    }

    pub fn ident(self, db: &::salsa::Db) -> Ident {
        match self.data(db) {
            ModulePathData::Root(crate_path) => crate_path.package_ident(db),
            ModulePathData::Child { parent: _, ident } => ident,
        }
    }

    pub fn raw_text(self, db: &::salsa::Db) -> &str {
        let diff_path = module_virtual_path(db, self).unwrap();
        db.file_from_virtual_path(diff_path)
            .unwrap()
            .text(db)
            .unwrap()
            .unwrap()
    }
}

impl PartialOrdWithDb for ModulePath {
    fn partial_cmp_with_db(&self, db: &::salsa::Db, other: &Self) -> Option<std::cmp::Ordering> {
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

#[test]
fn module_path_partial_ord_works() {
    let db = DB::default();
    let db = &*db;
    let path_menu = db.dev_path_menu().unwrap();

    assert!(path_menu.core_root().with_db(db) > (path_menu.core_num().inner()).with_db(db));
    assert!(!(path_menu.core_root().with_db(db) == (path_menu.core_num().inner()).with_db(db)));
    assert!(!(path_menu.core_root().with_db(db) < (path_menu.core_num().inner()).with_db(db)));
    assert!(!(path_menu.core_root().with_db(db) <= (path_menu.core_num().inner()).with_db(db)));
    assert!(path_menu.core_root().with_db(db) >= (path_menu.core_num().inner()).with_db(db));
    assert!(path_menu.core_root().with_db(db) != (path_menu.core_num().inner()).with_db(db));

    assert!(
        !(path_menu.core_prelude().inner().with_db(db) > path_menu.core_num().inner().with_db(db))
    );
    assert!(
        !(path_menu.core_prelude().inner().with_db(db) == path_menu.core_num().inner().with_db(db))
    );
    assert!(
        !(path_menu.core_prelude().inner().with_db(db) < path_menu.core_num().inner().with_db(db))
    );
    assert!(
        !(path_menu.core_prelude().inner().with_db(db) <= path_menu.core_num().inner().with_db(db))
    );
    assert!(
        !(path_menu.core_prelude().inner().with_db(db) >= path_menu.core_num().inner().with_db(db))
    );
    assert!(path_menu.core_prelude().with_db(db) != path_menu.core_num().with_db(db));

    assert_ne!(
        path_menu.std_root().with_db(db),
        path_menu.core_ops().inner().with_db(db),
    )
}

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModulePathData {
    Root(CratePath),
    Child { parent: ModulePath, ident: Ident },
}

impl ModulePathData {
    fn display(self, db: &::salsa::Db, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    pub fn to_string_with_db(&self, db: &::salsa::Db) -> String {
        self.display(db).to_string()
    }

    #[inline(never)]
    pub fn show(&self, f: &mut ::std::fmt::Formatter<'_>, db: &::salsa::Db) -> ::std::fmt::Result {
        f.write_str("`")?;
        self.show_aux(f, db)?;
        f.write_str("`")
    }

    #[inline(never)]
    pub fn show_aux(
        &self,
        f: &mut ::std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> ::std::fmt::Result {
        match self.data(db) {
            ModulePathData::Root(crate_path) => f.write_str(crate_path.package_ident(db).data(db)),
            ModulePathData::Child { parent, ident } => {
                parent.show_aux(f, db)?;
                f.write_str("::")?;
                f.write_str(ident.data(db))
            }
        }
    }
}

impl salsa::DisplayWithDb for ModulePath {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.show_aux(f, db)
    }
}

impl salsa::DisplayWithDb for SubmodulePath {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.inner().show_aux(f, db)
    }
}

#[test]
fn module_path_debug_with_db_works() {
    use salsa::DebugWithDb;
    fn t(db: &::salsa::Db, module_path: ModulePath, expect: &str) {
        assert_eq!(format!("{:?}", module_path.debug_with(db,)), expect)
    }
    let db = DB::default();
    let db = &*db;
    let path_menu = db.dev_path_menu().unwrap();
    t(db, path_menu.core_num().inner(), "`core::num`");
    t(db, path_menu.core_root(), "`core`");
    t(db, path_menu.std_root(), "`std`");
    expect_test::expect![[r#"
        `core`
    "#]]
    .assert_debug_eq(&path_menu.core_root().debug(db));
    expect_test::expect![[r#"
        SubmodulePath(
            `core::num`,
        )
    "#]]
    .assert_debug_eq(&path_menu.core_num().debug(db));
    expect_test::expect![[r#"
        `std`
    "#]]
    .assert_debug_eq(&path_menu.std_root().debug(db));
}

impl salsa::DebugWithDb for ModulePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, db: &::salsa::Db) -> std::fmt::Result {
        #[allow(unused_imports)]
        use ::salsa::debug::helper::Fallback;
        self.show(f, db)
    }
}
