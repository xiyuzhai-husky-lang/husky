use crate::*;
use std::process::Command;

impl CompilerInstance {
    pub(crate) fn cargo_fmt(&self, package_dir: &Path) {
        let output = Command::new("cargo")
            .current_dir(
                std::fs::canonicalize(package_dir)
                    .unwrap()
                    .join("__rust_gen_cache__"),
            )
            .arg("fmt")
            .output()
            .expect("failed to execute process");
        if !output.status.success() {
            panic!("{}", std::str::from_utf8(&output.stderr).unwrap())
        }
    }
}
