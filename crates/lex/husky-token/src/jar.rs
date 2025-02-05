use crate::*;

use husky_vfs::{path::module_path::ModulePath, script::Script};

#[deprecated(note = "create a trait HasTokenSheet")]
pub trait TokenDb {
    fn ranged_token_sheet(&self, module_path: ModulePath) -> &RangedTokenSheet;
    fn token_sheet_data(&self, module_path: ModulePath) -> &TokenSheetData;
    fn chunk_token_sheet_data(&self, script: Script) -> &TokenSheetData;
}

impl TokenDb for ::salsa::Db {
    fn token_sheet_data(&self, module_path: ModulePath) -> &TokenSheetData {
        token_sheet(self, module_path).data(self)
    }

    fn ranged_token_sheet(&self, module_path: ModulePath) -> &RangedTokenSheet {
        ranged_token_sheet(self, module_path)
    }

    fn chunk_token_sheet_data(&self, script: Script) -> &TokenSheetData {
        tokenize_snippet(self, script).token_sheet().data(self)
    }
}

#[salsa::jar]
pub struct TokenJar(
    TokenSheet,
    ranged_token_sheet,
    token_sheet,
    reserved_cowords,
    tokenize_snippet,
);
