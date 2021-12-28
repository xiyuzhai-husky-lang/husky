mod formatter;

pub type FormattedText = folded::FoldedList<ast::AstResult<String>>;

use ast::ExprError;

use file::FileResultArc;
use folded::Transformer;
use folded::{FoldedContainer, FoldedList};
use scope::ScopeQuery;
use std::sync::Arc;

use formatter::Formatter;

#[salsa::query_group(FormatQueryStorage)]
pub trait FmtQuery: ast::AstQuery {
    fn fmt_text(&self, id: file::FileId) -> ast::AstResultArc<FormattedText>;
}

fn fmt_text(db: &dyn FmtQuery, id: file::FileId) -> ast::AstResultArc<FormattedText> {
    let ast_text = db.ast_text(id)?;
    let mut formatter = Formatter::new(db.word_interner(), &ast_text.arena);
    formatter.transform_all(ast_text.folded_results.folded_iter(0));
    Ok(Arc::new(formatter.finish()))
}
