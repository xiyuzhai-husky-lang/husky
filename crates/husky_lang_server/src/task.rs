use vfs::FileID;

pub(crate) enum Task {
    Response(lsp_server::Response),
    Diagnostics(Vec<(FileID, Vec<lsp_types::Diagnostic>)>),
}

impl std::fmt::Debug for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Response(_) => f.write_str("Response"),
            Self::Diagnostics(arg0) => f.debug_tuple("Diagnostics").field(arg0).finish(),
        }
    }
}
