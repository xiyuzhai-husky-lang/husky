use crate::*;
use std::process::Command;

impl CompilerInstance {
    pub(crate) fn cargo_build(&self, package_dir: &Path) {
        println!("building {:?}", package_dir);
        let output = Command::new("cargo")
            .current_dir(
                std::fs::canonicalize(package_dir)
                    .unwrap()
                    .join("__rust_gen__"),
            )
            .args(["build", "--release"])
            .output()
            .expect("failed to execute process");
        if !output.status.success() {
            panic!("{}", std::str::from_utf8(&output.stderr).unwrap())
        }
    }
}
