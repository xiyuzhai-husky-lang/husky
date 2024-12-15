use std::{cell::Cell, ptr::NonNull};

use attach::Attach;

use crate::db::EternerDb;

thread_local! {
    /// The thread-local state salsa requires for a given thread
    static ATTACHED: Attached = const { Attached::new() }
}

/// State that is specific to a single execution thread.
///
/// Internally, this type uses ref-cells.
///
/// **Note also that all mutations to the database handle (and hence
/// to the local-state) must be undone during unwinding.**
struct Attached {
    /// Pointer to the currently attached database.
    database: Cell<Option<NonNull<EternerDb>>>,
}

impl Attached {
    const fn new() -> Self {
        Self {
            database: Cell::new(None),
        }
    }

    fn attach<R>(&self, db: &EternerDb, op: impl FnOnce() -> R) -> R {
        struct DbGuard<'s> {
            state: Option<&'s Attached>,
        }

        impl<'s> DbGuard<'s> {
            fn new(attached: &'s Attached, db: &EternerDb) -> Self {
                if let Some(current_db) = attached.database.get() {
                    let new_db = NonNull::from(db);

                    // Already attached? Assert that the database has not changed.
                    // NOTE: It's important to use `addr_eq` here because `NonNull::eq`
                    // not only compares the address but also the type's metadata.
                    if !std::ptr::addr_eq(current_db.as_ptr(), new_db.as_ptr()) {
                        panic!(
                            "Cannot change database mid-query. current: {current_db:?}, new: {new_db:?}",
                        );
                    }

                    Self { state: None }
                } else {
                    // Otherwise, set the database.
                    attached.database.set(Some(NonNull::from(db)));
                    Self {
                        state: Some(attached),
                    }
                }
            }
        }

        impl Drop for DbGuard<'_> {
            fn drop(&mut self) {
                // Reset database to null if we did anything in `DbGuard::new`.
                if let Some(attached) = self.state {
                    attached.database.set(None);
                }
            }
        }

        let _guard = DbGuard::new(self, db);
        op()
    }

    /// Access the "attached" database. Returns `None` if no database is attached.
    /// Databases are attached with `attach_database`.
    fn with<R>(&self, op: impl FnOnce(&EternerDb) -> R) -> Option<R> {
        let db = self.database.get()?;

        // SAFETY: We always attach the database in for the entire duration of a function,
        // so it cannot become "unattached" while this function is running.
        Some(op(unsafe { db.as_ref() }))
    }
}

impl Attach for &EternerDb {
    fn attach<R>(self, op: impl FnOnce() -> R) -> R {
        ATTACHED.with(|a| a.attach(self, op))
    }
}

/// Access the "attached" database. Returns `None` if no database is attached.
/// Databases are attached with `attach_database`.
pub fn with_attached_eterner_db<R>(op: impl FnOnce(&EternerDb) -> R) -> Option<R> {
    ATTACHED.with(|a| a.with(op))
}
