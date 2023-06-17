#[cfg(test)]
mod tests;
mod visibility;

pub use self::visibility::*;

use husky_token::TokenIdxRange;
use husky_vfs::{ModulePath, VfsDb};
use std::cmp::Ordering;
use with_db::PartialOrdWithDb;

/// Visibility is greater if it can be accessed from more places
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = VfsDb)]
pub enum Scope {
    Pub,                  // everyone can access it
    PubUnder(ModulePath), // everyone under a path can access it
    Private(ModulePath),  // only self
    Disconnected {
        module_path: ModulePath,
        file_visibility: FileScope,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileScope {
    token_idx_range: TokenIdxRange,
}

impl<Db: VfsDb + ?Sized> PartialOrdWithDb<Db> for Scope {
    fn partial_cmp_with_db(&self, db: &Db, other: &Self) -> Option<Ordering> {
        let db = <Db as salsa::DbWithJar<husky_vfs::VfsJar>>::as_jar_db(db);
        match (self, other) {
            (Scope::Pub, Scope::Pub) => Some(Ordering::Equal),
            (Scope::Pub, _) => Some(Ordering::Greater),
            (Scope::PubUnder(_), Scope::Pub) => Some(Ordering::Less),
            (Scope::PubUnder(module_path0), Scope::PubUnder(module_path1)) => {
                module_path0.partial_cmp_with_db(db, module_path1)
            }
            (Scope::PubUnder(_), Scope::Private(_) | Scope::Disconnected { .. }) => {
                Some(Ordering::Greater)
            }
            (Scope::Private(_), Scope::Pub) => {
                todo!()
            }
            (Scope::Private(_), Scope::PubUnder(_)) => todo!(),
            (Scope::Private(module_path1), Scope::Private(module_path2))
                if module_path1 == module_path2 =>
            {
                Some(Ordering::Equal)
            }
            (Scope::Private(_), Scope::Private(_)) => None,
            (
                Scope::Private(module_path1),
                Scope::Disconnected {
                    module_path: module_path2,
                    ..
                },
            ) if module_path1 == module_path2 => Some(Ordering::Greater),
            (Scope::Private(_), Scope::Disconnected { .. }) => None,
            (Scope::Disconnected { .. }, _) => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum ReferenceModulePath {
    Specific(ModulePath),
    Generic,
}

impl Scope {
    pub fn is_visible_from(
        self,
        db: &dyn VfsDb,
        reference_module_path: ReferenceModulePath,
    ) -> bool {
        match reference_module_path {
            ReferenceModulePath::Specific(reference_module_path) => match self {
                Scope::Pub => true,
                Scope::PubUnder(parent_module) => {
                    reference_module_path.starts_with(db, parent_module)
                }
                Scope::Private(module_path) => module_path == reference_module_path,
                Scope::Disconnected { .. } => todo!(),
            },
            ReferenceModulePath::Generic => todo!(),
        }
    }
}
