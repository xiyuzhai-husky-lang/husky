mod formatter;

pub type FormattedText = folded::FoldedList<Result<String, ExprError>>;

use ast::ExprError;

use file::FileResultArc;
use folded::Generator;
use folded::{FoldedList, FoldedStorage};
use scope::ScopeQuery;
use std::sync::Arc;

#[salsa::query_group(FormatQueryStorage)]
pub trait FormatQuery: ast::AstQuery {
    fn format_text(&self, id: file::FileId) -> FileResultArc<FormattedText>;
}

fn format_text(this: &dyn FormatQuery, id: file::FileId) -> FileResultArc<FormattedText> {
    todo!()
}
