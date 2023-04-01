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
    Pub,                  // everyone can access it
    PubUnder(ModulePath), // everyone under a path can access it
    Private(ModulePath),  // only self
    Disconnected {
        module_path: ModulePath,
        file_visibility: FileVisibility,
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
            (Visibility::Pub, Visibility::Pub) => Some(Ordering::Equal),
            (Visibility::Pub, _) => Some(Ordering::Greater),
            (Visibility::PubUnder(_), Visibility::Pub) => Some(Ordering::Less),
            (Visibility::PubUnder(module_path0), Visibility::PubUnder(module_path1)) => {
                module_path0.partial_cmp_with_db(db, module_path1)
            }
            (Visibility::PubUnder(_), Visibility::Private(_) | Visibility::Disconnected { .. }) => {
                Some(Ordering::Greater)
            }
            (Visibility::Private(_), Visibility::Pub) => {
                todo!()
            }
            (Visibility::Private(_), Visibility::PubUnder(_)) => todo!(),
            (Visibility::Private(module_path1), Visibility::Private(module_path2))
                if module_path1 == module_path2 =>
            {
                Some(Ordering::Equal)
            }
            (Visibility::Private(_), Visibility::Private(_)) => None,
            (
                Visibility::Private(module_path1),
                Visibility::Disconnected {
                    module_path: module_path2,
                    ..
                },
            ) if module_path1 == module_path2 => Some(Ordering::Greater),
            (Visibility::Private(_), Visibility::Disconnected { .. }) => None,
            (Visibility::Disconnected { .. }, _) => todo!(),
        }
    }
}

impl Visibility {
    pub fn is_visible_from(self, db: &dyn VfsDb, target_module_path: ModulePath) -> bool {
        match self {
            Visibility::Pub => true,
            Visibility::PubUnder(parent_module) => {
                target_module_path.starts_with(db, parent_module)
            }
            Visibility::Private(module_path) => module_path == target_module_path,
            Visibility::Disconnected { .. } => todo!(),
        }
    }
}
