//! Project loading & configuration updates
use std::{mem, sync::Arc};

use flycheck::{FlycheckConfig, FlycheckHandle};
use hir::db::DefDatabase;
use ide::Change;
use ide_db::base_db::{
    CrateGraph, Env, ProcMacro, ProcMacroExpander, ProcMacroExpansionError, ProcMacroKind,
    SourceRoot, VfsPath,
};
use vfs::{file_set::FileSetConfig, AbsPath, AbsPathBuf, ChangeKind};

use crate::{config::ServerConfig, lsp_ext, server::Server, task::Task};

#[derive(Debug)]
pub(crate) enum BuildDataProgress {
    Begin,
    Report(String),
    End(),
}

impl Server {
    pub(crate) fn update_configuration(&mut self, config: ServerConfig) {
        let _p = profile::span("Server::update_configuration");
        let old_config = mem::replace(&mut self.config, Arc::new(config));
    }

    pub(crate) fn current_status(&self) -> lsp_ext::ServerStatusParams {
        let mut status = lsp_ext::ServerStatusParams {
            health: lsp_ext::Health::Ok,
            quiescent: self.is_quiescent(),
            message: None,
        };
        status
    }

    fn reload_flycheck(&mut self) {
        todo!()
    }
}

#[derive(Default)]
pub(crate) struct ProjectFolders {
    pub(crate) load: Vec<vfs::loader::Entry>,
    pub(crate) watch: Vec<usize>,
}

pub(crate) fn should_refresh_for_change(path: &AbsPath, change_kind: ChangeKind) -> bool {
    const IMPLICIT_TARGET_FILES: &[&str] = &["build.rs", "src/main.rs", "src/lib.rs"];
    const IMPLICIT_TARGET_DIRS: &[&str] = &["src/bin", "examples", "tests", "benches"];
    let file_name = path.file_name().unwrap_or_default();

    if file_name == "Cargo.toml" || file_name == "Cargo.lock" {
        return true;
    }
    if change_kind == ChangeKind::Modify {
        return false;
    }
    if path.extension().unwrap_or_default() != "rs" {
        return false;
    }
    if IMPLICIT_TARGET_FILES
        .iter()
        .any(|it| path.as_ref().ends_with(it))
    {
        return true;
    }
    let parent = match path.parent() {
        Some(it) => it,
        None => return false,
    };
    if IMPLICIT_TARGET_DIRS
        .iter()
        .any(|it| parent.as_ref().ends_with(it))
    {
        return true;
    }
    if file_name == "main.rs" {
        let grand_parent = match parent.parent() {
            Some(it) => it,
            None => return false,
        };
        if IMPLICIT_TARGET_DIRS
            .iter()
            .any(|it| grand_parent.as_ref().ends_with(it))
        {
            return true;
        }
    }
    false
}
