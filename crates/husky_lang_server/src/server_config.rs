use project::{discover_projects, Project};
use vfs::AbsPathBuf;

use crate::Result;

#[derive(Debug)]
pub struct ServerConfig {
    pub(crate) root_path: AbsPathBuf,
    pub(crate) projects: Vec<Project>,
}

impl ServerConfig {
    pub fn detached_files(&self) -> &[AbsPathBuf] {
        todo!()
    }
    fn update(&mut self, mut json: serde_json::Value) {
        todo!()
    }

    pub fn new(init_params: lsp_types::InitializeParams) -> Result<ServerConfig> {
        trace_client_info(init_params.client_info);

        let root_path = get_root_path_from_initialize_params(init_params.root_uri)?;

        let mut config = ServerConfig {
            projects: {
                let workspace_roots =
                    get_workspace_roots(init_params.workspace_folders, &root_path);

                let projects = discover_projects(&workspace_roots);
                tracing::info!("discovered projects: {:?}", projects);
                if projects.is_empty() {
                    tracing::error!("failed to find any projects in {:?}", workspace_roots);
                }
                projects
            },
            root_path,
        };

        if let Some(json) = init_params.initialization_options {
            config.update(json);
        }

        Ok(config)
    }
}

fn get_workspace_roots(
    workspace_folders: Option<Vec<lsp_types::WorkspaceFolder>>,
    root_path: &AbsPathBuf,
) -> Vec<AbsPathBuf> {
    workspace_folders
        .map(|workspaces| {
            workspaces
                .into_iter()
                .filter_map(|it| it.uri.to_file_path().ok())
                .filter_map(|it| vfs::AbsPathBuf::try_from(it).ok())
                .collect::<Vec<_>>()
        })
        .filter(|workspaces| !workspaces.is_empty())
        .unwrap_or_else(|| vec![root_path.clone()])
}

fn trace_client_info(client_info: Option<lsp_types::ClientInfo>) {
    if let Some(client_info) = client_info {
        tracing::info!(
            "Client '{}' {}",
            client_info.name,
            client_info.version.unwrap_or_default()
        );
    }
}

fn get_root_path_from_initialize_params(
    root_uri: Option<lsp_types::Url>,
) -> crate::Result<AbsPathBuf> {
    Ok(
        match root_uri
            .and_then(|it| it.to_file_path().ok())
            .and_then(|it| AbsPathBuf::try_from(it).ok())
        {
            Some(it) => it,
            None => {
                let cwd = std::env::current_dir()?;
                AbsPathBuf::assert(cwd)
            }
        },
    )
}
