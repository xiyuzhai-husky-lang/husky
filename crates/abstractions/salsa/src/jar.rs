use crate::{
    storage::{HasJar, JarFromJars},
    Database, DbWithJar,
};

use super::routes::Routes;

pub trait Jar<'db>: Sized {
    type DynDb: ?Sized + HasJar<Self> + Database + 'db;

    fn initialize<DB>(&mut self, routes: &mut Routes<DB>)
    where
        DB: JarFromJars<Self> + DbWithJar<Self>;

    #[cfg(feature = "test-utils")]
    fn cast_test_db(db: &'db crate::test_utils::TestDb) -> &'db Self::DynDb;
}
