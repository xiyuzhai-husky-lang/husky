use crate::*;
use husky_defn::DefnDb;
use salsa::DbWithJar;

pub trait TokenInferDb: DbWithJar<TokenInferJar> + DefnDb {
    fn token_infer_sheet(&self, module_path: ModulePath) -> VfsResult<&TokenInferSheet>;
}

impl<Db> TokenInferDb for Db
where
    Db: DbWithJar<TokenInferJar> + DefnDb,
{
    fn token_infer_sheet(&self, module_path: ModulePath) -> VfsResult<&TokenInferSheet> {
        Ok(token_infer_sheet(self, module_path).as_ref()?)
    }
}
