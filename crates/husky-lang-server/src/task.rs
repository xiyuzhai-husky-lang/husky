use vfs::FileId;

#[derive(Debug)]
pub(crate) enum PrimeCachesProgress {
    Begin,
    Report(ide::PrimeCachesProgress),
    End { cancelled: bool },
}

pub(crate) mod ide {
    #[derive(Debug)]
    pub struct PrimeCachesProgress {}
}

#[derive(Debug)]
pub(crate) struct BuildDataProgress {}
#[derive(Debug)]
pub(crate) struct ProjectWorkspaceProgress {}

pub(crate) enum Task {
    Response(lsp_server::Response),
    Diagnostics(Vec<(FileId, Vec<lsp_types::Diagnostic>)>),
    PrimeCaches(PrimeCachesProgress),
    FetchWorkspace(ProjectWorkspaceProgress),
    FetchBuildData(BuildDataProgress),
}

impl std::fmt::Debug for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Response(_) => f.write_str("Response"),
            Self::Diagnostics(arg0) => f.debug_tuple("Diagnostics").field(arg0).finish(),
            Self::PrimeCaches(arg0) => f.debug_tuple("PrimeCaches").field(arg0).finish(),
            Self::FetchWorkspace(arg0) => f.debug_tuple("FetchWorkspace").field(arg0).finish(),
            Self::FetchBuildData(arg0) => f.debug_tuple("FetchBuildData").field(arg0).finish(),
        }
    }
}
