use cargo::core::{Summary, Workspace};
use std::path::Path;

pub fn workspace_package_summaries(workspace_dir: &Path) -> Vec<Summary> {
    let manifest_path = &workspace_dir.join("Cargo.toml");
    let config = cargo::Config::default().expect("what the hell");
    let ws = Workspace::new(manifest_path, &config).expect("what the hell");
    ws.members().map(|pkg| pkg.summary().clone()).collect()
}
