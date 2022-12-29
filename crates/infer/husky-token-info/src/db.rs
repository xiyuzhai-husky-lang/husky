use crate::*;
use husky_defn::DefnDb;
use salsa::DbWithJar;

pub trait TokenInferDb: DbWithJar<TokenInfoJar> + DefnDb {
    fn token_info_sheet(&self, module_path: ModulePath) -> EntityTreeResult<&TokenInfoSheet>;
}

impl<Db> TokenInferDb for Db
where
    Db: DbWithJar<TokenInfoJar> + DefnDb,
{
    fn token_info_sheet(&self, module_path: ModulePath) -> EntityTreeResult<&TokenInfoSheet> {
        Ok(token_info_sheet(self, module_path).as_ref()?)
    }
}
