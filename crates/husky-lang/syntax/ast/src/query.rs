use fold::Transformer;
use fold::{FoldStorage, FoldedList};
use std::sync::Arc;

use crate::{error::AstResultArc, *};

#[salsa::query_group(AstQueryStorage)]
pub trait AstQuery: scope::ScopeQuery {
    fn ast_text(&self, id: file::FileId) -> scope::ScopeResultArc<AstText>;
}

fn ast_text(this: &dyn AstQuery, id: file::FileId) -> scope::ScopeResultArc<AstText> {
    let tokenized_text = this.tokenized_text(id)?;
    let mut parser = AstTransformer::new(this, this.module_from_file_id(id)?);
    parser.transform_all(tokenized_text.fold_iter(0));
    Ok(Arc::new(parser.finish()))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstText {
    pub arena: ExprArena,
    pub folded_results: FoldedList<AstResult<Ast>>,
}

impl AstText {
    pub fn errors(&self) -> Vec<&AstError> {
        self.folded_results
            .nodes
            .iter()
            .filter_map(|node| node.value.as_ref().err())
            .collect()
    }
}
