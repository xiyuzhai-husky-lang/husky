use crate::*;

use husky_token_data::db::TokenDataDb;
use husky_vfs::{snippet::Snippet, *};

use salsa::DbWithJar;

pub trait TokenDb: DbWithJar<TokenJar> + VfsDb + TokenDataDb {
    fn ranged_token_sheet(&self, module_path: ModulePath) -> &RangedTokenSheet;
    fn token_sheet_data(&self, module_path: ModulePath) -> &TokenSheetData;
    fn snippet_token_sheet_data(&self, snippet: Snippet) -> &TokenSheetData;
}

impl<T> TokenDb for T
where
    T: DbWithJar<TokenJar> + VfsDb + TokenDataDb,
{
    fn token_sheet_data(&self, module_path: ModulePath) -> &TokenSheetData {
        token_sheet(self, module_path).data(self)
    }

    fn ranged_token_sheet(&self, module_path: ModulePath) -> &RangedTokenSheet {
        ranged_token_sheet(self, module_path)
    }

    fn snippet_token_sheet_data(&self, snippet: Snippet) -> &TokenSheetData {
        tokenize_snippet(self, snippet).token_sheet().data(self)
    }
}

#[salsa::jar(db = TokenDb)]
pub struct TokenJar(
    TokenSheet,
    ranged_token_sheet,
    token_sheet,
    reserved_cowords,
    tokenize_snippet,
);
