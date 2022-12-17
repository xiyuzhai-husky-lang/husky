use crate::*;
use husky_absolute_path::AbsolutePath;
use husky_ast::AstDb;
use husky_check_utils::should;
use husky_dev_utils::dev_src;
use husky_entity_card::{EntityCard, MemberKind, TyKingdom};
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

pub trait EntityTreeDb: DbWithJar<EntityTreeJar> + AstDb {
    fn entity_absolute_path(
        &self,
        entity_path: EntityPath,
    ) -> &EntityTreeResult<AbsoluteEntityPath>;
    fn entity_tree_sheet(&self, module: EntityPath) -> &VfsResult<EntityTreeSheet>;
    fn entity_card(&self, entity_path: EntityPath) -> &EntityTreeResult<EntityCard>;
    fn is_absolute(&self, entity_path: EntityPath) -> EntityTreeResult<bool> {
        Ok(self.entity_absolute_path(entity_path).as_ref()?.path() == entity_path)
    }
}

impl<T> EntityTreeDb for T
where
    T: DbWithJar<EntityTreeJar> + AstDb,
{
    fn entity_absolute_path(
        &self,
        entity_path: EntityPath,
    ) -> &EntityTreeResult<AbsoluteEntityPath> {
        absolute_entity_path(self, entity_path)
    }

    fn entity_card(&self, entity_path: EntityPath) -> &EntityTreeResult<EntityCard> {
        entity_card(self, entity_path)
    }

    fn entity_tree_sheet(&self, module: EntityPath) -> &VfsResult<EntityTreeSheet> {
        entity_tree_sheet(self, module)
    }
}
