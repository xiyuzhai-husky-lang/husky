mod formatter;

use formatter::Formatter;
use husky_ast::AstContext;
use husky_entity_path::EntityPath;
use salsa::DbWithJar;
use std::sync::Arc;

#[salsa::jar(db = SyntaxFormatDb)]
pub struct SyntaxFormatJar();

pub trait SyntaxFormatDb: DbWithJar<SyntaxFormatJar> + husky_ast::AstDb {
    fn fmt_text(&self, id: EntityPath) -> husky_entity_tree::EntityTreeResultArc<String>;
}

impl<T> SyntaxFormatDb for T
where
    T: DbWithJar<SyntaxFormatJar> + husky_ast::AstDb,
{
    fn fmt_text(&self, id: EntityPath) -> husky_entity_tree::EntityTreeResultArc<String> {
        todo!()
    }
}

fn fmt_text(
    db: &dyn SyntaxFormatDb,
    file: EntityPath,
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
