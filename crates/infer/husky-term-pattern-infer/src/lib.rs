mod comptime;
mod context;
mod error;
mod query;
mod sheet;
#[cfg(test)]
mod tests;
mod ty;

pub use comptime::*;
pub use error::*;
pub use query::*;
pub use sheet::*;

use context::*;
use husky_expr_syntax::*;
use husky_file::*;
use husky_opn_syntax::*;
use husky_primitive_literal_syntax::RawLiteralData;
use husky_print_utils::p;
use husky_symbol_syntax::SymbolKind;
use husky_term::*;
use husky_term_pattern::*;
use husky_word::*;

impl<'a> TermPatternInferContext<'a> {
    fn infer_term_pattern(&self) -> TermPatternInferEntry {
        let comptime = self.infer_comptime();
        let ty = self.infer_ty();
        TermPatternInferEntry {
            const_expr: comptime,
            ty,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TermPatternInferEntry {
    const_expr: TermPatternInferResult<Option<ConstExprPattern>>,
    ty: TermPatternInferResult<TermPatternItd>,
}
