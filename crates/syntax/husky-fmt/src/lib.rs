mod formatter;

pub type FormattedText = fold::FoldableList<husky_ast::AstResult<String>>;

use fold::{Executor, FoldableStorage};
use husky_ast::AstContext;
use std::sync::Arc;

use formatter::Formatter;

#[salsa::query_group(FormatQueryGroupStorage)]
pub trait FmtQuery: husky_ast::AstDb {
    fn fmt_text(&self, id: husky_path::PathItd) -> husky_entity_tree::EntityTreeResultArc<String>;
}

fn fmt_text(
    db: &dyn FmtQuery,
    file: husky_path::PathItd,
) -> husky_entity_tree::EntityTreeResultArc<String> {
    todo!()
    // let ast_text = db.ast_text(file)?;
    // let mut formatter = Formatter::new(
    //     db.upcast(),
    //     &ast_text.arena,
    //     AstContext::Module(db.module(file).unwrap()),
    // );
    // formatter.execute_all(ast_text.folded_results.iter());
    // Ok(Arc::new(formatter.finish()))
}
