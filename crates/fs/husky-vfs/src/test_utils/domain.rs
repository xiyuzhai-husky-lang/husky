use super::*;

pub(super) struct VfsTestDomain {
    src_base: PathBuf,
    expect_files_base: PathBuf,
    adversarials_base: PathBuf,
}

impl VfsTestDomain {
    pub(super) fn new(
        src_base: PathBuf,
        expect_files_base: PathBuf,
        adversarials_base: PathBuf,
    ) -> Self {
        std::fs::create_dir_all(&expect_files_base).expect("failed_to_create_dir_all");
        std::fs::create_dir_all(&adversarials_base).expect("failed_to_create_dir_all");
        Self {
            src_base,
            expect_files_base,
            adversarials_base,
        }
    }

    pub(super) fn src_base(&self) -> &Path {
        &self.src_base
    }

    pub(super) fn expect_files_base(&self) -> &Path {
        &self.expect_files_base
    }

    pub(super) fn adversarials_base(&self) -> &Path {
        &self.adversarials_base
    }
}

pub(super) fn vfs_test_domains() -> Vec<VfsTestDomain> {
    let env = HuskyDevPathEnv::new();
    let dir = env
        .cargo_manifest_dir()
        .map(|p| p.to_owned())
        .unwrap_or("temp".into());
    vec![
        VfsTestDomain::new(
            env.lang_dev_library_dir().to_owned(),
            dir.join("expect-files/library"),
            dir.join("adversarials/library"),
        ),
        VfsTestDomain::new(
            env.lang_dev_examples_dir().to_owned(),
            dir.join("expect-files/examples"),
            dir.join("adversarials/examples"),
        ),
    ]
}
