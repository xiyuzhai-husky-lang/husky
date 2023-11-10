pub mod db;
pub mod deps;

use self::db::*;
use self::deps::*;
use husky_entity_path::{FugitivePath, ItemPath};
use husky_hir_ty::HirTemplateArgumentLiterals;

#[salsa::interned(db = LinkagePathDb, jar = LinkagePathJar, constructor = new)]
pub struct LinkagePath {
    data: LinkagePathData,
}

impl LinkagePath {
    pub fn new_suffix(db: &dyn LinkagePathDb) -> Self {
        todo!()
    }

    pub fn new_field(db: &dyn LinkagePathDb) -> Self {
        Self::new(db, LinkagePathData::Field)
    }

    pub fn new_item(
        db: &dyn LinkagePathDb,
        path: impl Into<ItemPath>,
        template_arguments: HirTemplateArgumentLiterals,
    ) -> Self {
        Self::new(
            db,
            LinkagePathData::Item {
                path: path.into(),
                template_arguments,
            },
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LinkagePathData {
    Coersion {},
    Item {
        path: ItemPath,
        template_arguments: HirTemplateArgumentLiterals,
    },
    Field,
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
