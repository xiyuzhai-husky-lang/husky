use husky_git_utils::dirty::assert_husky_git_dir_clean;
use husky_path_utils::{
    rust::{collect_rust_source_paths_with, husky_cargo_workspace_manifest_dir},
    Path,
};
use std::fs;

pub fn eliminate_mods() {
    assert_husky_git_dir_clean();
    for path in collect_rust_source_paths_with(husky_cargo_workspace_manifest_dir(), |path| {
        path.ends_with("mod.rs")
    }) {
        rename_mod_file(&path).unwrap()
    }
}

fn rename_mod_file(path: &Path) -> std::io::Result<()> {
    if path.ends_with("mod.rs") {
        if let Some(parent_path) = path.parent() {
            let new_path = parent_path.with_extension("rs");
            fs::rename(path, new_path)?;
        }
    }
    Ok(())
}
