use std::path::Path;

/// Represents the output of a Lean command execution
#[derive(Debug, Clone)]
pub struct LeanOutput {
    pub stdout: String,
    pub stderr: String,
}

/// Find the Lean package root directory by looking for lakefile.toml or lakefile.lean
/// in the ancestry of the given path
pub fn find_package_dir(path: &Path) -> Option<&Path> {
    path.ancestors()
        .find(|dir| dir.join("lakefile.toml").exists() || dir.join("lakefile.lean").exists())
}

/// Execute `lake lean` for path and return both stdout and stderr.
pub fn lake_lean(path: &Path) -> LeanOutput {
    let package_dir = find_package_dir(path)
        .expect("Failed to find Lean package directory (no lakefile.toml or lakefile.lean found)");

    let output = std::process::Command::new("lake")
        .arg("lean")
        .arg(path)
        .current_dir(package_dir)
        .output()
        .expect("Failed to execute lake lean");

    LeanOutput {
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    }
}

#[test]
fn lake_lean_works() {
    let dev_paths = husky_path_utils::HuskyLangDevPaths::new();
    let path = dev_paths
        .projects_dir()
        .join("ai-math-autoformalization/lean/central-46/Central46/Standalone.lean");
    let output = lake_lean(&path);
    println!("stdout:\n{}", output.stdout);
    println!("stderr:\n{}", output.stderr);
}
