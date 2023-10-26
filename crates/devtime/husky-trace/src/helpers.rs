use crate::*;

#[salsa::tracked(jar = TraceJar, return_ref)]
pub(crate) fn root_traces(db: &dyn TraceDb, crate_path: CratePath) -> Vec<Trace> {
    todo!()
}
