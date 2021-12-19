use crate::*;

use file::FileResultArc;
use folded::Parser;
use folded::{FoldedList, FoldedStorage};
use scope::ScopeQuery;
use std::sync::Arc;

pub type AtomizedText = FoldedList<AtomResult>;

#[salsa::query_group(AtomQueryStorage)]
pub trait AtomQuery: ScopeQuery {
    fn atomized_text(&self, id: file::FileId) -> FileResultArc<AtomizedText>;
}

fn atomized_text(this: &dyn AtomQuery, id: file::FileId) -> FileResultArc<AtomizedText> {
    let tokenized_text = this.tokenized_text(id)?;
    let mut parser = AtomParser::new(this);
    parser.parse_all(tokenized_text.folded_iter(0));
    Ok(Arc::new(parser.take_folded_results()))
}
