pub type AtomizedText = FoldedList<AtomParseResult>;
pub use error::AtomResultArc;

use common::*;

use crate::*;

use file::FileId;
use folded::Transformer;
use folded::{FoldedList, FoldedStorage};
use scope::ScopeQuery;
use std::sync::Arc;

#[salsa::query_group(AtomQueryStorage)]
pub trait AtomQuery: ScopeQuery {
    fn atomized_text(&self, id: FileId) -> AtomResultArc<AtomizedText>;
}

fn atomized_text(this: &dyn AtomQuery, id: FileId) -> AtomResultArc<AtomizedText> {
    let tokenized_text = this.tokenized_text(id)?;
    let module = this.module_from_file_id(id)?;
    let mut parser = AtomParser::new(this, module);
    parser.transform_all(tokenized_text.folded_iter(0));
    Ok(Arc::new(parser.take_folded_results()))
}
