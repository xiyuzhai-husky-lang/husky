use crate::*;

use husky_sema_expr::SemaExprDb;
use salsa::DbWithJar;

pub trait TokenInfoDb: DbWithJar<TokenInfoJar> + SemaExprDb {
    fn token_info_sheet(&self, module_path: ModulePath) -> EntitySynTreeResult<&TokenInfoSheet>;
}

impl<Db> TokenInfoDb for Db
where
    Db: DbWithJar<TokenInfoJar> + SemaExprDb,
{
    fn token_info_sheet(&self, module_path: ModulePath) -> EntitySynTreeResult<&TokenInfoSheet> {
        Ok(token_info_sheet(self, module_path)
            .as_ref()
            .map_err(|e| e.clone())?)
    }
}
