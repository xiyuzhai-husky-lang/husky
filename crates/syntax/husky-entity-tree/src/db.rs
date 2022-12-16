use crate::*;
use husky_absolute_path::AbsolutePath;
use husky_check_utils::should;
use husky_dev_utils::dev_src;
use husky_entity_kind::{EntityKind, MemberKind, TyKind};
use husky_entity_path::EntityPath;
use husky_package_path::PackagePath;
use husky_path_utils::*;
use husky_print_utils::msg_once;
use husky_term::*;
use husky_token::TokenDb;
use husky_vfs::VfsResult;
use husky_word::Identifier;
use salsa::DbWithJar;
use std::{path::PathBuf, sync::Arc};

pub trait EntityTreeDb: DbWithJar<EntityTreeJar> + TokenDb {
    fn absolute_entity_path(&self, entity_path: EntityPath) -> AbsoluteEntityPath;
    fn is_absolute(&self, entity_path: EntityPath) -> bool {
        self.absolute_entity_path(entity_path).path() == entity_path
    }
}

impl<T> EntityTreeDb for T
where
    T: DbWithJar<EntityTreeJar> + TokenDb,
{
    fn absolute_entity_path(&self, entity_path: EntityPath) -> AbsoluteEntityPath {
        absolute_entity_path(self, entity_path)
    }
}

fn entity_tree_sheet(db: &dyn EntityTreeDb, entity_path: EntityPath) -> VfsResult<EntityNodeSheet> {
    todo!()
}
