use fold::Transformer;
use fold::{FoldStorage, FoldedList};
use scope_query::ScopeResultArc;
use std::sync::Arc;

use crate::atom::symbol_proxy::{Symbol, SymbolProxy};
use crate::*;

#[salsa::query_group(AstQueryGroupStorage)]
pub trait AstQueryGroup: scope_query::ScopeQueryGroup {
    fn ast_text(&self, id: file::FilePtr) -> ScopeResultArc<AstText>;

    fn parse_ty(&self, code: &'static str) -> AstResult<ScopePtr>;
}

fn ast_text(this: &dyn AstQueryGroup, id: file::FilePtr) -> ScopeResultArc<AstText> {
    let tokenized_text = this.tokenized_text(id)?;
    let mut parser = AstTransformer::new(this, this.module_from_file_id(id)?);
    parser.transform_all(tokenized_text.fold_iter(0));
    Ok(Arc::new(parser.finish()))
}

fn parse_ty(this: &dyn AstQueryGroup, code: &'static str) -> AstResult<ScopePtr> {
    let tokens = this.tokenize(code);
    let symbols = fold::LocalStack::<Symbol>::new();
    let proxy = SymbolProxy {
        main: None,
        db: this,
        symbols: &symbols,
    };
    atom::parser::parse_ty(proxy, &tokens)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstText {
    pub arena: RawExprArena,
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
