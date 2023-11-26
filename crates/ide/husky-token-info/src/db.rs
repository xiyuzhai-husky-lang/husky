use crate::*;

use husky_sema_expr::SemaExprDb;

pub trait TokenInfoDb: DbWithJar<TokenInfoJar> + SemaExprDb {
    #[deprecated]
    fn token_info_sheet(&self, module_path: ModulePath) -> EntitySynTreeResult<&TokenInfoSheet>;
    fn token_info_sheet_ref(
        &self,
        module_path: ModulePath,
    ) -> EntitySynTreeResult<TokenInfoSheetRef>;
}

impl TokenInfoDb for Db
where
    Db: DbWithJar<TokenInfoJar> + SemaExprDb,
{
    fn token_info_sheet(&self, module_path: ModulePath) -> EntitySynTreeResult<&TokenInfoSheet> {
        Ok(token_info_sheet(self, module_path)
            .as_ref()
            .map_err(|e| e.clone())?)
    }

    fn token_info_sheet_ref(
        &self,
        module_path: ModulePath,
    ) -> EntitySynTreeResult<TokenInfoSheetRef> {
        Ok(self.token_info_sheet(module_path)?.to_ref())
    }
}

#[salsa::jar(db =  TokenInfoDb)]
pub struct TokenInfoJar(token_info_sheet);
