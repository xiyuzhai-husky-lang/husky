use crate::*;

use file::FileResultArc;
use folded::Generator;
use folded::{FoldedList, FoldedStorage};
use scope::ScopeQuery;
use std::sync::Arc;

#[salsa::query_group(ExprQueryStorage)]
pub trait FmtQuery: expr::ExprQuery {
    fn expr_text(&self, id: file::FileId) -> FileResultArc<FormattedText>;
}

fn expr_text(this: &dyn FmtQuery, id: file::FileId) -> FileResultArc<FormattedText> {
    todo!()
}
