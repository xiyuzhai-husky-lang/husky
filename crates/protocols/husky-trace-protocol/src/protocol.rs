use std::path::{Path, PathBuf};

use husky_task_interface::pedestal::IsPedestal;

use crate::*;

pub trait IsTraceProtocol:
    Default + std::fmt::Debug + Clone + PartialEq + Eq + Send + 'static
{
    type Pedestal: IsPedestal;
    type Figure: IsFigure;
}

impl IsTraceProtocol for () {
    type Pedestal = ();

    type Figure = ();
}

pub trait IsTraceProtocolFull: IsTraceProtocol + Serialize + for<'a> Deserialize<'a> {}

impl<T> IsTraceProtocolFull for T where T: IsTraceProtocol + Serialize + for<'a> Deserialize<'a> {}

pub trait IsTrace: std::fmt::Debug + Eq + Copy + From<TraceId> + Into<TraceId> {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceBundle<Trace: IsTrace> {
    crate_root_module_file_abs_path: PathBuf,
    root_traces: Vec<Trace>,
}

impl<Trace: IsTrace> TraceBundle<Trace> {
    pub fn new(crate_root_module_file_abs_path: PathBuf, root_traces: Vec<Trace>) -> Self {
        Self {
            crate_root_module_file_abs_path,
            root_traces,
        }
    }

    pub fn crate_root_module_file_abs_path(&self) -> &Path {
        &self.crate_root_module_file_abs_path
    }

    pub fn root_traces(&self) -> &[Trace] {
        self.root_traces.as_ref()
    }
}

pub enum TraceKindProtocol {
    LazyCall,
    LazyExpr,
    LazyStmt,
    EagerCall,
    EagerExpr,
    EagerStmt,
}
