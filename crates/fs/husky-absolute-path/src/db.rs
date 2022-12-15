use crate::*;
use salsa::DbWithJar;

pub trait AbsolutePathDb: DbWithJar<AbsolutePathJar> {
    fn absolute_path_db(&self) -> &dyn AbsolutePathDb;
}

impl<T> AbsolutePathDb for T
where
    T: DbWithJar<AbsolutePathJar>,
{
    fn absolute_path_db(&self) -> &dyn AbsolutePathDb {
        self
    }
}
