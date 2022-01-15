mod formatter;

pub type FormattedText = fold::FoldedList<ast::AstResult<String>>;

use fold::{Executor, FoldStorage};
use std::sync::Arc;

use formatter::Formatter;

#[salsa::query_group(FormatQueryGroupStorage)]
pub trait FmtQuery: ast::AstQueryGroup {
    fn fmt_text(&self, id: file::FileId) -> scope::ScopeResultArc<String>;
}

fn fmt_text(db: &dyn FmtQuery, id: file::FileId) -> scope::ScopeResultArc<String> {
    let ast_text = db.ast_text(id)?;
    let mut formatter = Formatter::new(db.word_interner(), &ast_text.arena);
    formatter.execute_all(ast_text.folded_results.fold_iter(0));
    Ok(Arc::new(formatter.finish()))
}
