use cargo::core::{Summary, Workspace};
use std::path::Path;

pub fn workspace_package_summaries(workspace_dir: &Path) -> Vec<Summary> {
    let manifest_path = &workspace_dir.join("Cargo.toml");
    let gctx = cargo::GlobalContext::default().expect("what the hell");
    let ws = Workspace::new(manifest_path, &gctx).expect("what the hell");
    ws.members().map(|pkg| pkg.summary().clone()).collect()
}
