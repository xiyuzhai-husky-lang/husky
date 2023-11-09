pub mod db;
pub mod deps;

use self::db::*;
use self::deps::*;
use husky_entity_path::{FugitivePath, ItemPath};
use husky_hir_ty::HirTemplateArgumentLiterals;

#[salsa::interned(db = LinkagePathDb, jar = LinkagePathJar)]
pub struct LinkagePath {
    data: LinkagePathData,
}

impl LinkagePath {
    pub fn new_suffix(db: &dyn LinkagePathDb) -> Self {
        todo!()
    }

    pub fn new_item(db: &dyn LinkagePathDb) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LinkagePathData {
    Coersion {},
    Item {
        path: ItemPath,
        template_arguments: HirTemplateArgumentLiterals,
    },
    Todo,
}

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
