use crate::{parser::ExprParser, *};

use file::FileResultArc;
use folded::Transformer;
use folded::{FoldedList, FoldedStorage};
use std::sync::Arc;

pub type ExprText = FoldedList<ExprResult>;

#[salsa::query_group(ExprQueryStorage)]
pub trait ExprQuery: atom::AtomQuery {
    fn expr_text(&self, id: file::FileId) -> FileResultArc<ExprText>;
}

fn expr_text(this: &dyn ExprQuery, id: file::FileId) -> FileResultArc<ExprText> {
    let atomized_text = this.atomized_text(id)?;
    let mut parser = ExprParser::new();
    parser.convert_all(atomized_text.folded_iter(0));
    Ok(Arc::new(parser.take_folded_results()))
}
