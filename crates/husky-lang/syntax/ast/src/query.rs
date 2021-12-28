use folded::Transformer;
use folded::{FoldedContainer, FoldedList};
use std::sync::Arc;

use crate::{
    error::AstResultArc,
    transform::{AstGenResult, AtomToAstTransformer},
    *,
};

#[salsa::query_group(AstQueryStorage)]
pub trait AstQuery: atom::AtomQuery {
    fn ast_text(&self, id: file::FileId) -> AstResultArc<AstText>;
}

fn ast_text(this: &dyn AstQuery, id: file::FileId) -> AstResultArc<AstText> {
    let atomized_text = this.atomized_text(id)?;
    let mut parser = AtomToAstTransformer::new();
    parser.transform_all(atomized_text.folded_iter(0));
    Ok(Arc::new(parser.finish()))
}
