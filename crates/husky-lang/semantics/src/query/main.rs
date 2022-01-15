use fold::FoldStorage;

use crate::{error::*, stmt::LazyStmtQueryGroup, *};

#[salsa::query_group(MainQueryGroupStorage)]
pub trait MainQueryGroup: ast::AstQueryGroup + LazyStmtQueryGroup {
    fn main(&self, main_file: file::FileId) -> SemanticResultArc<Main>;
}

fn main(this: &dyn MainQueryGroup, main_file: file::FileId) -> SemanticResultArc<Main> {
    let ast_text = this.ast_text(main_file)?;
    for item in ast_text.folded_results.fold_iter(0) {
        match item.value.as_ref()? {
            ast::Ast::MainDef => {
                return Ok(Arc::new(Main {
                    stmts: this.parse_lazy_stmts(&ast_text.arena, not_none!(item.children))?,
                }))
            }
            _ => (),
        }
    }
    err!()
}
