use std::sync::Arc;

use crate::*;
use ast::AstQueryGroup;
use entity_route_query::EntityRouteQueryGroup;
use file::FilePtr;
use infer_total::InferQueryGroup;

#[salsa::query_group(HighlightQueryGroupStorage)]
pub trait HighlightQueryGroup: EntityRouteQueryGroup + AstQueryGroup + InferQueryGroup {
    fn highlights(&self, file: FilePtr) -> Arc<Vec<Highlight>>;
}

fn highlights(db: &dyn HighlightQueryGroup, file: FilePtr) -> Arc<Vec<Highlight>> {
    todo!()
}
