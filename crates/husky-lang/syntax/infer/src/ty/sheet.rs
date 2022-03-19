mod builder;
mod deduction;
mod entry;

use arena::ArenaMap;
use ast::RawExpr;
use builder::TySheetBuilder;
use deduction::Deduction;
use entry::TySheetEntry;

use super::*;

pub(crate) fn ty_sheet(db: &dyn InferSalsaQueryGroup, file: FilePtr) -> SyntaxResultArc<TySheet> {
    let ast_text = db.ast_text(file)?;
    let mut ty_sheet_builder = TySheetBuilder::new(db);
    ty_sheet_builder.infer_all(ast_text.folded_results.fold_iter(0));
    Ok(Arc::new(ty_sheet_builder.finish()))
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct TySheet {
    data: ArenaMap<RawExpr, TySheetEntry>,
}

impl TySheet {
    pub(crate) fn ty(&self, expr_idx: RawExprIdx) -> SyntaxResult<ScopePtr> {
        todo!()
    }
}
