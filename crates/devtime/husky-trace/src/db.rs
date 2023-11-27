use crate::*;

pub trait TraceDb {
    fn root_traces(&self, crate_path: CratePath) -> &[TraceId];
}

impl TraceDb for ::salsa::Db {
    fn root_traces(&self, crate_path: CratePath) -> &[TraceId] {
        crate::trace::root_traces(self, crate_path).as_ref()
    }
}

#[salsa::jar(db = TraceDb)]
pub struct TraceJar(
    crate::trace::TracePath,
    crate::trace::TraceId,
    crate::trace::trace_view_lines,
    crate::trace::trace_have_subtraces,
    crate::trace::trace_subtraces,
    crate::trace::trace_val_repr_expansion,
    crate::trace::submodule::submodule_contains_val_item,
    // helpers
    crate::trace::root_traces,
);
