use crate::*;

use husky_vfs::{path::module_path::ModulePath, script::Script};

pub trait TokenDb {
    fn ranged_token_sheet(&self, module_path: ModulePath) -> &RangedTokenSheet;
    fn token_sheet_data(&self, module_path: ModulePath) -> &TokenSheetData;
    fn snippet_token_sheet_data(&self, snippet: Script) -> &TokenSheetData;
}

impl TokenDb for ::salsa::Db {
    fn token_sheet_data(&self, module_path: ModulePath) -> &TokenSheetData {
        token_sheet(self, module_path).data(self)
    }

    fn ranged_token_sheet(&self, module_path: ModulePath) -> &RangedTokenSheet {
        ranged_token_sheet(self, module_path)
    }

    fn snippet_token_sheet_data(&self, snippet: Script) -> &TokenSheetData {
        tokenize_snippet(self, snippet).token_sheet().data(self)
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
