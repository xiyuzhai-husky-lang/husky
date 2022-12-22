use std::cmp::Ordering;

use super::*;

/// Accessibility is greater if it can be accessed from more places
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Accessibility {
    Public,                  // everyone can access it
    PublicUnder(ModulePath), // everyone under a path can access it
    Private,                 // only self
}

impl PartialOrd for Accessibility {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Accessibility::Public, Accessibility::Public) => Some(Ordering::Equal),
            (Accessibility::Public, Accessibility::PublicUnder(_) | Accessibility::Private) => {
                Some(Ordering::Greater)
            }
            (Accessibility::PublicUnder(_), Accessibility::Public) => Some(Ordering::Less),
            (Accessibility::PublicUnder(_), Accessibility::PublicUnder(_)) => todo!(),
            (Accessibility::PublicUnder(_), Accessibility::Private) => todo!(),
            (Accessibility::Private, Accessibility::Public) => todo!(),
            (Accessibility::Private, Accessibility::PublicUnder(_)) => todo!(),
            (Accessibility::Private, Accessibility::Private) => todo!(),
        }
    }
}

#[test]
fn accessibility_partial_ord_works() {
    use Accessibility::*;

    let db = DB::default();
    let toolchain = db.dev_toolchain();
    let path_menu = db.path_menu(toolchain).unwrap();
    assert!(Public > PublicUnder(path_menu.core_num()));
    assert!(!(PublicUnder(path_menu.core_prelude()) > PublicUnder(path_menu.core_num())));
    assert!(Public > Private);
    assert!(Public >= Public);
}

impl Accessibility {
    pub fn is_accessible_from(self, db: &dyn VfsDb, module_path: ModulePath) -> bool {
        match self {
            Accessibility::Public => true,
            Accessibility::PublicUnder(parent_module) => module_path.starts_with(db, parent_module),
            Accessibility::Private => todo!(),
        }
    }
}

impl salsa::DebugWithDb<dyn VfsDb + '_> for Accessibility {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn VfsDb,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        match self {
            Self::Public => write!(f, "Public"),
            Self::PublicUnder(module_path) => f
                .debug_tuple("PubicUnder")
                .field(&module_path.debug_with(db, include_all_fields))
                .finish(),
            Self::Private => write!(f, "Private"),
        }
    }
}

impl<Db: VfsDb> salsa::DebugWithDb<Db> for Accessibility {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}
