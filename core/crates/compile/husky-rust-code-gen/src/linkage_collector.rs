use crate::*;
use husky_linkage_table::LinkageKey;
use std::collections::HashMap;
mod context;

pub use context::*;

pub(crate) struct LinkageCollector<'a> {
    db: &'a dyn RustCodeGenQueryGroup,
    linkages: HashMap<LinkageKey, String>,
    context: LinkageCollectorContext,
}

impl<'a> LinkageCollector<'a> {
    pub(crate) fn collect_all(mut self) -> () {
        todo!()
    }
}
