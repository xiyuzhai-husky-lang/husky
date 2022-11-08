mod context;
mod error;
mod expr;
mod query;
mod sheet;
#[cfg(test)]
mod tests;
mod ty;

pub use error::*;
use husky_primitive_literal_syntax::RawLiteralData;
use husky_print_utils::p;
use husky_symbol_syntax::SymbolKind;
pub use query::*;
pub use sheet::*;

use context::*;
use husky_expr_syntax::*;
use husky_file::*;
use husky_opn_syntax::*;
use husky_term::*;
use husky_term_pattern::*;
use husky_word::*;

impl<'a> TermPatternInferContext<'a> {
    fn infer_term_pattern(&self) -> TermPatternInferEntry {
        let expr_term_pattern = self.infer_expr_term_pattern();
        let ty_expr_term_pattern = self.infer_ty_term_pattern();
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TermPatternInferEntry {
    expr_term_pattern: Option<TermPatternInferResult<TermPatternItd>>,
    ty_term_pattern: TermPatternInferResult<TermPatternItd>,
}
