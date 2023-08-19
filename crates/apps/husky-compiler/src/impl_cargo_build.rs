use crate::*;
use std::process::Command;

pub enum CargoBuildM {
    Ok,
}

pub struct CargoBuildR;

impl std::ops::FromResidual<CargoBuildR> for CargoBuildM {
    fn from_residual(_residual: CargoBuildR) -> Self {
        todo!()
    }
}

impl std::ops::Try for CargoBuildM {
    type Output = ();

    type Residual = CargoBuildR;

    fn from_output(_output: Self::Output) -> Self {
        todo!()
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            CargoBuildM::Ok => std::ops::ControlFlow::Continue(()),
        }
    }
}

impl CompilerInstance {
    pub(crate) fn cargo_build(&self, package_dir: &Path) -> CargoBuildM {
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
        CargoBuildM::Ok
    }
}
