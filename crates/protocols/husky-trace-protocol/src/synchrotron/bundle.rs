use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceIdBundle {
    crate_root_module_file_abs_path: PathBuf,
    root_trace_ids: Vec<TraceId>,
}

impl TraceIdBundle {
    pub(super) fn new(
        crate_root_module_file_abs_path: PathBuf,
        root_trace_ids: Vec<TraceId>,
    ) -> Self {
        Self {
            crate_root_module_file_abs_path,
            root_trace_ids,
        }
    }

    pub fn crate_root_module_file_abs_path(&self) -> &Path {
        &self.crate_root_module_file_abs_path
    }

    pub fn root_trace_ids(&self) -> &[TraceId] {
        self.root_trace_ids.as_ref()
    }
}
