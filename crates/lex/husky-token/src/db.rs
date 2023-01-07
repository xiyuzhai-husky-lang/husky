use crate::*;

use husky_vfs::*;

use salsa::DbWithJar;

pub trait TokenDb: DbWithJar<TokenJar> + VfsDb {
    fn ranged_token_sheet(&self, module_path: ModulePath) -> VfsResult<&RangedTokenSheet>;
    fn token_sheet_data(&self, module_path: ModulePath) -> VfsResult<&TokenSheetData>;
    fn snippet_token_sheet_data(&self, snippet: Snippet) -> &TokenSheetData;
}

impl<T> TokenDb for T
where
    T: DbWithJar<TokenJar> + VfsDb,
{
    fn token_sheet_data(&self, module_path: ModulePath) -> VfsResult<&TokenSheetData> {
        Ok(token_sheet(self, module_path)?.data(self))
    }

    fn ranged_token_sheet(&self, module_path: ModulePath) -> VfsResult<&RangedTokenSheet> {
        Ok(ranged_token_sheet(self, module_path).as_ref()?)
    }

    fn snippet_token_sheet_data(&self, snippet: Snippet) -> &TokenSheetData {
        tokenize_snippet(self, snippet).token_sheet().data(self)
    }
}
