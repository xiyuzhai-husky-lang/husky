pub type AtomicText = FoldedList<AtomParseResult>;
pub use error::AtomResultArc;

use common::*;

use crate::*;

use file::FileId;
use folded::Transformer;
use folded::{FoldedContainer, FoldedList};
use scope::ScopeQuery;
use std::sync::Arc;

#[salsa::query_group(AtomQueryStorage)]
pub trait AtomQuery: ScopeQuery {
    fn atomized_text(&self, id: FileId) -> AtomResultArc<AtomicText>;
}

fn atomized_text(this: &dyn AtomQuery, id: FileId) -> AtomResultArc<AtomicText> {
    let tokenized_text = this.tokenized_text(id)?;
    let module = this.module_from_file_id(id)?;
    let mut parser = AtomicTransformer::new(this, module);
    parser.transform_all(tokenized_text.folded_iter(0));
    Ok(Arc::new(parser.take_folded_results()))
}
