mod calc;
mod error;
#[cfg(test)]
mod tests;

pub use error::*;

use calc::*;
use husky_ast::AstDb;

use husky_token::TokenDb;
use husky_vfs::*;
use lsp_types::FoldingRange;

pub trait FoldingRangeDb {
    fn folding_ranges(&self, module: ModulePath) -> &[FoldingRange];
}

impl FoldingRangeDb for ::salsa::Db {
    fn folding_ranges(&self, module_path: ModulePath) -> &[FoldingRange] {
        folding_ranges(self, module_path)
    }
}

#[salsa::jar(db = FoldingRangeDb)]
pub struct FoldingRangeJar(folding_ranges);

#[salsa::tracked(jar = FoldingRangeJar, return_ref)]
fn folding_ranges(db: &::salsa::Db, module_path: ModulePath) -> Vec<FoldingRange> {
    let ast_sheet = db.ast_sheet(module_path);
    let ast_range_sheet = db.ast_token_idx_range_sheet(module_path);
    let ranged_token_sheet = db.ranged_token_sheet(module_path);
    calc_folding_ranges(ast_sheet, ast_range_sheet, ranged_token_sheet)
}
