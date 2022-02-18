use common::Upcast;
use fold::FoldStorage;

use crate::{error::*, *};

#[salsa::query_group(MainQueryGroupStorage)]
pub trait MainQueryGroup:
    ast::AstQueryGroup + InferQueryGroup + Upcast<dyn InferQueryGroup>
{
    fn main(&self, main_file: file::FilePtr) -> SemanticResultArc<Main>;
}

fn main(this: &dyn MainQueryGroup, main_file: file::FilePtr) -> SemanticResultArc<Main> {
    let ast_text = this.ast_text(main_file)?;
    for item in ast_text.folded_results.fold_iter(0) {
        match item.value.as_ref()? {
            ast::Ast::MainDef => {
                return Ok(Arc::new(Main {
                    stmts: stmt::parse_decl_stmts(
                        this.upcast(),
                        &ast_text.arena,
                        not_none!(item.children),
                        main_file,
                    )?,
                }))
            }
            _ => (),
        }
    }
    err!("main not found")
}
