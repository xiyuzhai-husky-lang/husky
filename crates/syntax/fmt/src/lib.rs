mod formatter;

pub type FormattedText = fold::FoldedList<ast::AstResult<String>>;

use ast::AstContext;
use fold::{Executor, FoldStorage};
use std::sync::Arc;

use formatter::Formatter;

#[salsa::query_group(FormatQueryGroupStorage)]
pub trait FmtQuery: ast::AstQueryGroup {
    fn fmt_text(&self, id: file::FilePtr) -> entity_route_query::ScopeResultArc<String>;
}

fn fmt_text(db: &dyn FmtQuery, file: file::FilePtr) -> entity_route_query::ScopeResultArc<String> {
    let ast_text = db.ast_text(file)?;
    let mut formatter = Formatter::new(
        db.word_allocator(),
        &ast_text.arena,
        AstContext::Module(db.module(file).unwrap()),
    );
    formatter.execute_all(ast_text.folded_results.iter());
    Ok(Arc::new(formatter.finish()))
}
