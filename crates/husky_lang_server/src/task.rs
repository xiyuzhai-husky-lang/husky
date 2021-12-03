use vfs::FileID;

#[derive(Debug)]
pub(crate) enum PrimeCachesProgress {
    Begin,
    Report(ide::PrimeCachesProgress),
    End { cancelled: bool },
}

#[derive(Debug)]
pub(crate) struct BuildDataProgress {}
#[derive(Debug)]
pub(crate) struct ProjectWorkspaceProgress {}

pub(crate) enum Task {
    Response(lsp_server::Response),
    Diagnostics(Vec<(FileID, Vec<lsp_types::Diagnostic>)>),
    PrimeCaches(PrimeCachesProgress),
}

impl std::fmt::Debug for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Response(_) => f.write_str("Response"),
            Self::Diagnostics(arg0) => f.debug_tuple("Diagnostics").field(arg0).finish(),
            Self::PrimeCaches(arg0) => f.debug_tuple("PrimeCaches").field(arg0).finish(),
        }
    }
}
