use crate::*;
use husky_expr::ExprDb;
use salsa::DbWithJar;

pub trait TokenInferDb: DbWithJar<TokenInferJar> + ExprDb {
    fn token_infer_sheet(&self, module_path: ModulePath) -> VfsResult<&TokenInferSheet>;
}

impl<Db> TokenInferDb for Db
where
    Db: DbWithJar<TokenInferJar> + ExprDb,
{
    fn token_infer_sheet(&self, module_path: ModulePath) -> VfsResult<&TokenInferSheet> {
        Ok(token_infer_sheet(self, module_path).as_ref()?)
    }
}
