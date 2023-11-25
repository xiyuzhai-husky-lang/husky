use husky_text::db::TextDb;
use husky_token_info::TokenInfoDb;

use crate::*;

pub trait TraceDb: salsa::DbWithJar<TraceJar> + TokenInfoDb + TextDb + ValReprDb {
    fn root_traces(&self, crate_path: CratePath) -> &[Trace];
}

impl<Db> TraceDb for Db
where
    Db: salsa::DbWithJar<TraceJar> + TokenInfoDb + TextDb + ValReprDb,
{
    fn root_traces(&self, crate_path: CratePath) -> &[Trace] {
        crate::trace::root_traces(self, crate_path).as_ref()
    }
}

#[salsa::jar(db = TraceDb)]
pub struct TraceJar(
    crate::trace::TracePath,
    crate::trace::Trace,
    crate::trace::trace_view_lines,
    crate::trace::trace_have_subtraces,
    crate::trace::trace_subtraces,
    crate::trace::trace_val_repr_expansion,
    crate::trace::submodule::submodule_contains_val_item,
    // helpers
    crate::trace::root_traces,
);
