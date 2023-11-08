pub mod db;
pub mod deps;

use self::db::*;
use self::deps::*;

#[salsa::interned(db = LinkagePathDb, jar = LinkagePathJar)]
pub struct LinkagePath {
    data: LinkagePathData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LinkagePathData {}

impl LinkagePath {
    pub fn deps(self, db: &dyn LinkagePathDb) -> LinkageDeps {
        todo!()
    }
}
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum LinkagePath {
//     FnCall {},
//     MethodFnCall {},
//     FieldAccess {},
// }

// impl LinkagePath {
//     pub fn deps(self, db: &dyn HirDepsDb) -> LinkageDeps {
//         todo!()
//     }
// }
