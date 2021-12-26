use folded::Generator;
use folded::{FoldedList, FoldedStorage};
use std::sync::Arc;

use crate::{expr::ExprParser, expr::ExprResultArc, *};

pub type AST = FoldedList<ExprResult>;

#[salsa::query_group(AstQueryStorage)]
pub trait AstQuery: atom::AtomQuery {
    fn expr_text(&self, id: file::FileId) -> ExprResultArc<AST>;
}

fn expr_text(this: &dyn AstQuery, id: file::FileId) -> ExprResultArc<AST> {
    let atomized_text = this.atomized_text(id)?;
    let mut parser = ExprParser::new();
    parser.transform_all(atomized_text.folded_iter(0));
    Ok(Arc::new(parser.take_folded_results()))
}
