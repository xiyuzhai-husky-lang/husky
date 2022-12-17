use crate::*;
use husky_ast::AstSheet;
use std::collections::HashSet;

pub struct EntityTreeBuilder<'a> {
    db: &'a dyn EntityTreeDb,
    ast_sheet: &'a AstSheet,
    modified_modules: HashSet<EntityPath>,
}

impl<'a> EntityTreeBuilder<'a> {
    pub fn new(db: &'a dyn EntityTreeDb, ast_sheet: &'a AstSheet) -> Self {
        Self {
            db,
            ast_sheet,
            modified_modules: Default::default(),
        }
    }

    pub fn build(mut self) -> VfsResult<EntityTreeSheet> {
        // let entity_ast_sheet = entity_ast_sheet(self.db, root);
        todo!()
    }
}
