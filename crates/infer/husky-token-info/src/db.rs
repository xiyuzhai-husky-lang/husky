use crate::*;
use husky_defn::DefnDb;
use husky_expr_ty::ExprTypeDb;
use salsa::DbWithJar;

pub trait TokenInfoDb: DbWithJar<TokenInfoJar> + ExprTypeDb {
    fn token_info_sheet(&self, module_path: ModulePath) -> EntityTreeResult<&TokenInfoSheet>;
}

impl<Db> TokenInfoDb for Db
where
    Db: DbWithJar<TokenInfoJar> + ExprTypeDb,
{
    fn token_info_sheet(&self, module_path: ModulePath) -> EntityTreeResult<&TokenInfoSheet> {
        Ok(token_info_sheet(self, module_path)
            .as_ref()
            .map_err(|e| e.clone())?)
    }
}
