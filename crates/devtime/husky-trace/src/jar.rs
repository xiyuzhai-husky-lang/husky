use crate::*;
use husky_trace_protocol::protocol::TraceBundle;
use husky_vfs::path::crate_path::CratePath;

pub trait TraceDb {
    fn trace_bundles(&self, crate_path: CratePath) -> &[TraceBundle<Trace>];
}

impl TraceDb for ::salsa::Db {
    fn trace_bundles(&self, crate_path: CratePath) -> &[TraceBundle<Trace>] {
        trace_bundles(self, crate_path)
    }
}

#[salsa::jar]
pub struct TraceJar(
    crate::trace::TracePath,
    crate::trace::Trace,
    crate::trace::trace_view_lines,
    crate::trace::trace_have_subtraces,
    crate::trace::trace_subtraces,
    crate::trace::trace_ki_repr_expansion,
    crate::trace::trace_var_deps,
    crate::trace::submodule::submodule_contains_val,
    // helpers
    crate::trace::root_traces,
    crate::trace::trace_bundles,
);
