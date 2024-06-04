pub mod ancestry;
pub mod relative_path;

use crate::script::Script;

use super::*;
pub use ancestry::*;
use maybe_result::*;
use salsa::{AsId, DisplayWithDb};
use with_db::PartialOrdWithDb;
#[cfg(test)]
use with_db::WithDb;

#[salsa::interned(jar = VfsJar, constructor = new_inner, override_debug)]
pub struct ModulePath {
    pub data: ModulePathData,
}

#[salsa::as_id]
#[salsa::deref_id]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScriptModulePath {
    module_path: ModulePath,
}

impl ScriptModulePath {
    pub fn module_path(self) -> ModulePath {
        self.module_path
    }

    pub fn new(script: Script, db: &::salsa::Db) -> Self {
        Self {
            module_path: ModulePath::new_inner(db, ModulePathData::Script { script }),
        }
    }

    pub fn script(self, db: &::salsa::Db) -> Script {
        let ModulePathData::Script { script } = self.module_path.data(db) else {
            unreachable!()
        };
        script
    }
}

impl ModulePath {
    pub fn new(db: &::salsa::Db, data: ModulePathData) -> VfsMaybeResult<Self> {
        let slf = Self::new_inner(db, data);
        if let Some(virtual_path) = module_virtual_path(db, slf)? {
            if virtual_path.file(db)?.text(db)?.is_none() {
                return Nothing;
            }
        }
        JustOk(slf)
    }

    pub fn is_root(self, db: &::salsa::Db) -> bool {
        match self.data(db) {
            ModulePathData::Root(_) => true,
            ModulePathData::Child { .. } => false,
            ModulePathData::Script { .. } => false,
        }
    }

    pub fn root_module_path(self, db: &::salsa::Db) -> Self {
        match self.data(db) {
            ModulePathData::Root(_) | ModulePathData::Script { .. } => self,
            ModulePathData::Child { .. } => self.module_ancestry(db).root_module_path(),
        }
    }
}

/// wrapper type that guarantees that the inner field is a submodule
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
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
            ModulePathData::Script { .. } => None,
        }
    }

    /// use CratePath::root_module_path instead in other crates
    pub(crate) fn new_root(db: &::salsa::Db, crate_path: CratePath) -> VfsMaybeResult<Self> {
        Self::new(db, ModulePathData::Root(crate_path))
    }

    pub fn new_child(
        db: &::salsa::Db,
        parent: ModulePath,
        ident: Ident,
    ) -> VfsMaybeResult<SubmodulePath> {
        JustOk(SubmodulePath(Self::new(
            db,
            ModulePathData::Child { parent, ident },
        )?))
    }

    pub fn toolchain(self, db: &::salsa::Db) -> Toolchain {
        self.crate_path(db).toolchain(db)
    }

    #[inline(always)]
    pub fn virtual_path(self, db: &::salsa::Db) -> Option<VirtualPath> {
        module_virtual_path(db, self).expect("guaranteed")
    }

    #[inline(always)]
    pub fn abs_path(self, db: &::salsa::Db) -> Option<PathBuf> {
        self.virtual_path(db)
            .map(|virtual_path| virtual_path.abs_path(db).unwrap())
    }

    pub fn ident(self, db: &::salsa::Db) -> Ident {
        match self.data(db) {
            ModulePathData::Root(crate_path) => crate_path.package_ident(db),
            ModulePathData::Child { ident, .. } => ident,
            ModulePathData::Script { script: snippet } => Ident::from_ref(db, "snippet").unwrap(),
        }
    }

    pub fn raw_text(self, db: &::salsa::Db) -> &str {
        match module_virtual_path(db, self).unwrap() {
            Some(virtual_path) => db
                .file_from_virtual_path(virtual_path)
                .unwrap()
                .text(db)
                .unwrap()
                .unwrap(),
            None => match self.data(db) {
                ModulePathData::Root(_) | ModulePathData::Child { .. } => unreachable!(),
                ModulePathData::Script { script } => script.data(db),
            },
        }
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

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModulePathData {
    Root(CratePath),
    Child { parent: ModulePath, ident: Ident },
    Script { script: Script },
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
            ModulePathData::Script { script: snippet } => {
                f.write_fmt(format_args!("{}", snippet.as_id().as_u32()))
            }
        }
    }
}

impl salsa::DisplayWithDb for ModulePath {
    fn display_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.show_aux(f, db)
    }
}

impl salsa::DisplayWithDb for SubmodulePath {
    fn display_fmt_with_db(
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
    ::expect_test::expect![[r#"
        `core`
    "#]]
    .assert_debug_eq(&path_menu.core_root().debug(db));
    ::expect_test::expect![[r#"
        SubmodulePath(
            `core::num`,
        )
    "#]]
    .assert_debug_eq(&path_menu.core_num().debug(db));
    ::expect_test::expect![[r#"
        `std`
    "#]]
    .assert_debug_eq(&path_menu.std_root().debug(db));
}

impl salsa::DebugWithDb for ModulePath {
    fn debug_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        #[allow(unused_imports)]
        use ::salsa::debug::helper::Fallback;
        self.show(f, db)
    }
}
