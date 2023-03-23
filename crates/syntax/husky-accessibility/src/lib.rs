#[cfg(test)]
mod tests;

use husky_token::TokenIdxRange;
use husky_vfs::{ModulePath, VfsDb};
use std::cmp::Ordering;
use with_db::PartialOrdWithDb;

/// Visibility is greater if it can be accessed from more places
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = VfsDb)]
pub enum Visibility {
    Public,                  // everyone can access it
    PublicUnder(ModulePath), // everyone under a path can access it
    Private,                 // only self
    Disconnected {
        module_path: ModulePath,
        file_accessibility: FileVisibility,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileVisibility {
    token_idx_range: TokenIdxRange,
}

impl<Db: VfsDb + ?Sized> PartialOrdWithDb<Db> for Visibility {
    fn partial_cmp_with_db(&self, db: &Db, other: &Self) -> Option<Ordering> {
        let db = <Db as salsa::DbWithJar<husky_vfs::VfsJar>>::as_jar_db(db);
        match (self, other) {
            (Visibility::Public, Visibility::Public) => Some(Ordering::Equal),
            (Visibility::Public, _) => Some(Ordering::Greater),
            (Visibility::PublicUnder(_), Visibility::Public) => Some(Ordering::Less),
            (Visibility::PublicUnder(module_path0), Visibility::PublicUnder(module_path1)) => {
                module_path0.partial_cmp_with_db(db, module_path1)
            }
            (Visibility::PublicUnder(_), Visibility::Private | Visibility::Disconnected { .. }) => {
                Some(Ordering::Greater)
            }
            (Visibility::Private, Visibility::Public) => {
                todo!()
            }
            (Visibility::Private, Visibility::PublicUnder(_)) => todo!(),
            (Visibility::Private, Visibility::Private) => Some(Ordering::Equal),
            (Visibility::Private, Visibility::Disconnected { .. }) => Some(Ordering::Greater),
            (Visibility::Disconnected { .. }, _) => todo!(),
        }
    }
}

impl Visibility {
    pub fn is_visible_from(self, db: &dyn VfsDb, module_path: ModulePath) -> bool {
        match self {
            Visibility::Public => true,
            Visibility::PublicUnder(parent_module) => module_path.starts_with(db, parent_module),
            Visibility::Private => todo!(),
            Visibility::Disconnected { .. } => todo!(),
        }
    }
}
