use std::cmp::Ordering;

use with_db::{PartialOrdWithDb, WithDb};

use super::*;

/// Accessibility is greater if it can be accessed from more places
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Accessibility {
    Public,                  // everyone can access it
    PublicUnder(ModulePath), // everyone under a path can access it
    Private,                 // only self
}

impl PartialOrdWithDb<dyn VfsDb + '_> for Accessibility {
    fn partial_cmp_with_db(&self, db: &dyn VfsDb, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Accessibility::Public, Accessibility::Public) => Some(Ordering::Equal),
            (Accessibility::Public, _) => Some(Ordering::Greater),
            (Accessibility::PublicUnder(_), Accessibility::Public) => Some(Ordering::Less),
            (
                Accessibility::PublicUnder(module_path0),
                Accessibility::PublicUnder(module_path1),
            ) => module_path0.partial_cmp_with_db(db, module_path1),
            (Accessibility::PublicUnder(_), Accessibility::Private) => todo!(),
            (Accessibility::Private, Accessibility::Public) => todo!(),
            (Accessibility::Private, Accessibility::PublicUnder(_)) => todo!(),
            (Accessibility::Private, Accessibility::Private) => Some(Ordering::Equal),
        }
    }
}

impl<Db: VfsDb> PartialOrdWithDb<Db> for Accessibility {
    fn partial_cmp_with_db(&self, db: &Db, other: &Self) -> Option<std::cmp::Ordering> {
        self.partial_cmp_with_db(db as &dyn VfsDb, other)
    }
}

#[test]
fn accessibility_partial_ord_works() {
    use Accessibility::*;

    let db = DB::default();
    let path_menu = db.dev_path_menu().unwrap();
    assert!(Public.with_db(&db) > PublicUnder(path_menu.core_num()).with_db(&db));
    assert!(
        !(PublicUnder(path_menu.core_prelude()).with_db(&db)
            > PublicUnder(path_menu.core_num()).with_db(&db))
    );
    assert!(Public.with_db(&db) > Private.with_db(&db));
    assert!(Public.with_db(&db) >= Public.with_db(&db));
    // equals
    assert_eq!(Public.with_db(&db), Public.with_db(&db));
    assert_eq!(Private.with_db(&db), Private.with_db(&db));
    assert_eq!(
        PublicUnder(path_menu.core_prelude()).with_db(&db),
        PublicUnder(path_menu.core_prelude()).with_db(&db)
    );
    // not equals
    assert_ne!(Public.with_db(&db), Private.with_db(&db));
    assert_ne!(Private.with_db(&db), Public.with_db(&db));
    assert_ne!(
        Private.with_db(&db),
        PublicUnder(path_menu.core_num()).with_db(&db)
    );
    assert_ne!(
        PublicUnder(path_menu.core_prelude()).with_db(&db),
        PublicUnder(path_menu.core_num()).with_db(&db)
    );
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
