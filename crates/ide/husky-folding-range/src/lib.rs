mod calc;
mod error;
#[cfg(test)]
mod tests;

pub use error::*;

use calc::*;
use husky_ast::AstDb;

use husky_vfs::*;
use lsp_types::FoldingRange;

pub trait FoldingRangeDb: salsa::DbWithJar<FoldingRangeJar> + AstDb {
    fn folding_ranges(&self, module: ModulePath) -> FoldingRangeResult<&[FoldingRange]>;
}

impl<T> FoldingRangeDb for T
where
    T: salsa::DbWithJar<FoldingRangeJar> + AstDb,
{
    fn folding_ranges(&self, module_path: ModulePath) -> FoldingRangeResult<&[FoldingRange]> {
        Ok(folding_ranges(self, module_path).as_ref()?)
    }
}

#[salsa::jar(db = FoldingRangeDb)]
pub struct FoldingRangeJar(folding_ranges);

#[salsa::tracked(jar = FoldingRangeJar, return_ref)]
fn folding_ranges(
    db: &dyn FoldingRangeDb,
    module_path: ModulePath,
) -> FoldingRangeResult<Vec<FoldingRange>> {
    let ast_sheet = db.ast_sheet(module_path)?;
    let ast_range_sheet = db.ast_range_sheet(module_path)?;
    let ranged_token_sheet = db.ranged_token_sheet(module_path)?;
    Ok(calc_folding_ranges(
        ast_sheet,
        ast_range_sheet,
        ranged_token_sheet,
    ))
}
