use crate::*;

use husky_vfs::{*};

pub trait AstDb {
    #[deprecated(note = "use HasAstSheet trait instead")]
    fn ast_sheet(&self, module_path: ModulePath) -> &AstSheet;
    #[deprecated(note = "use HasAstSheet trait instead")]
    fn ast_token_idx_range_sheet(&self, module_path: ModulePath) -> &AstTokenIdxRangeSheet;
}

impl AstDb for ::salsa::Db {
    fn ast_sheet(&self, module_path: ModulePath) -> &AstSheet {
        ast_sheet(self, module_path)
    }

    fn ast_token_idx_range_sheet(&self, module_path: ModulePath) -> &AstTokenIdxRangeSheet {
        ast_token_idx_range_sheet(self, module_path)
    }
}
