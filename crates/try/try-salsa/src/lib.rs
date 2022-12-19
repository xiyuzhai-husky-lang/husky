mod word_separator;

#[salsa::jar(db = Db)]
pub struct Jar();

pub trait Db: salsa::DbWithJar<Jar> {}

impl<T> Db for T where T: salsa::DbWithJar<Jar> {}

#[salsa::db(Jar)]
#[derive(Default)]
pub struct DB {
    storage: salsa::Storage<DB>,
}

impl salsa::Database for DB {}

impl salsa::ParallelDatabase for DB {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(DB {
            storage: self.storage.snapshot(),
        })
    }
}

impl std::panic::RefUnwindSafe for DB {}

#[test]
fn test_traits() {
    fn f<A>(a: &A)
    where
        A: std::panic::RefUnwindSafe,
    {
    }

    f::<DB>;
}
