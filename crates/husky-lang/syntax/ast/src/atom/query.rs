// pub type AtomicText = FoldedList<AstResult<Ast>>;
// pub use error::AstResultArc;

// use common::*;

// use crate::*;

// use file::FileId;
// use fold::Transformer;
// use fold::{FoldStorage, FoldedList};
// use scope::ScopeQuery;
// use std::sync::Arc;

// #[salsa::query_group(AtomQueryStorage)]
// pub trait AtomQuery: ScopeQuery {
//     fn atomized_text(&self, id: FileId) -> AstResultArc<AtomicText>;
// }

// fn atomized_text(this: &dyn AtomQuery, id: FileId) -> AstResultArc<AtomicText> {
//     let tokenized_text = this.tokenized_text(id)?;
//     let module = this.module_from_file_id(id)?;
//     let mut parser = AtomicTransformer::new(this, module);
//     parser.transform_all(tokenized_text.fold_iter(0));
//     Ok(Arc::new(parser.take_fold_results()))
// }
