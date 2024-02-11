use crate::*;

pub fn cargo_manifest_dir() -> Result<PathBuf, std::env::VarError> {
    std::env::var("CARGO_MANIFEST_DIR").map(|s| s.into())
}

pub fn husky_cargo_workspace_manifest_dir() -> Result<PathBuf, std::env::VarError> {
    let base_dir = cargo_manifest_dir()?;
    let mut dir: &Path = &base_dir;
    let mut last_manifest_dir: &Path = &dir;
    loop {
        let Some(parent_dir) = dir.parent() else {
            return Ok(last_manifest_dir.to_owned());
        };
        dir = parent_dir;
        let cargo_manifest_path = &dir.join("Cargo.toml");
        if cargo_manifest_path.exists() {
            assert!(cargo_manifest_path.is_file());
            last_manifest_dir = dir;
        };
    }
}
