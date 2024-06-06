use crate::*;

pub trait TokenInfoDb {
    #[deprecated]
    fn token_info_sheet(&self, module_path: ModulePath) -> EntityTreeResult<&TokenInfoSheet>;
    fn token_info_sheet_ref(&self, module_path: ModulePath) -> EntityTreeResult<TokenInfoSheetRef>;
}

impl TokenInfoDb for ::salsa::Db {
    fn token_info_sheet(&self, module_path: ModulePath) -> EntityTreeResult<&TokenInfoSheet> {
        Ok(token_info_sheet(self, module_path)
            .as_ref()
            .map_err(|e| e.clone())?)
    }

    fn token_info_sheet_ref(&self, module_path: ModulePath) -> EntityTreeResult<TokenInfoSheetRef> {
        Ok(self.token_info_sheet(module_path)?.to_ref())
    }
}

#[salsa::jar]
pub struct TokenInfoJar(token_info_sheet);
