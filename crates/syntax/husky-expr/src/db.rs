use crate::*;
use husky_ast::AstSheet;
use husky_entity_tree::EntityTreeDb;
use husky_vfs::VfsResult;
use salsa::DbWithJar;

pub trait ExprDb: DbWithJar<ExprJar> + EntityTreeDb {
    fn expr_sheet(&self, module_path: ModulePath) -> VfsResult<&ExprSheet>;
}

impl<Db> ExprDb for Db
where
    Db: DbWithJar<ExprJar> + EntityTreeDb,
{
    fn expr_sheet(&self, module_path: ModulePath) -> VfsResult<&ExprSheet> {
        Ok(expr_sheet(self, module_path).as_ref()?)
    }
}

pub(crate) struct ExprBuilder<'a> {
    db: &'a dyn ExprDb,
    ast: &'a AstSheet,
}
