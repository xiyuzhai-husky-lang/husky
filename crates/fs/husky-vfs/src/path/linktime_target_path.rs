use super::*;
use crate::workspace_path::WorkspacePath;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[enum_class::from_variants]
#[salsa::debug_with_db(db = VfsDb)]
pub enum LinktimeTargetPath {
    Package(PackagePath),
    Workspace(WorkspacePath),
}
