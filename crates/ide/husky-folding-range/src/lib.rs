mod calc;
mod error;
#[cfg(test)]
mod tests;

pub use error::*;

use calc::*;
use husky_ast::AstDb;
use husky_entity_path::EntityPath;
use lsp_types::FoldingRange;

pub trait FoldingRangeDb: salsa::DbWithJar<FoldingRangeJar> + AstDb {
    fn folding_ranges(&self, module: EntityPath) -> &FoldingRangeResult<Vec<FoldingRange>>;
}

impl<T> FoldingRangeDb for T
where
    T: salsa::DbWithJar<FoldingRangeJar> + AstDb,
{
    fn folding_ranges(&self, module: EntityPath) -> &FoldingRangeResult<Vec<FoldingRange>> {
        folding_ranges(self, module)
    }
}

#[salsa::jar(db = FoldingRangeDb)]
pub struct FoldingRangeJar(folding_ranges);

#[salsa::tracked(jar = FoldingRangeJar, return_ref)]
fn folding_ranges(
    db: &dyn FoldingRangeDb,
    module: EntityPath,
) -> FoldingRangeResult<Vec<FoldingRange>> {
    let ast_sheet = db.ast_sheet(module).as_ref()?;
    let ast_range_sheet = db.ast_range_sheet(module).as_ref()?;
    Ok(calc_folding_ranges(ast_sheet, ast_range_sheet))
}
