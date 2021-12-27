use folded::Generator;
use folded::{FoldedList, FoldedStorage};
use std::sync::Arc;

use crate::{
    error::AstResultArc,
    gen::{AstGenResult, AstGenerator},
};

pub type AST = FoldedList<AstGenResult>;

#[salsa::query_group(AstQueryStorage)]
pub trait AstQuery: atom::AtomQuery {
    fn ast_text(&self, id: file::FileId) -> AstResultArc<AST>;
}

fn ast_text(this: &dyn AstQuery, id: file::FileId) -> AstResultArc<AST> {
    let atomized_text = this.atomized_text(id)?;
    let mut parser = AstGenerator::new();
    parser.transform_all(atomized_text.folded_iter(0));
    Ok(Arc::new(parser.take_folded_results()))
}
