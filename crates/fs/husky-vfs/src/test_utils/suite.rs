use super::*;

#[derive(Debug)]
#[salsa::debug_with_db(db = VfsDb, jar = VfsJar)]
pub(super) struct VfsTestSuite {
    src_base: PathBuf,
    expect_files_base: PathBuf,
    adversarials_base: Option<PathBuf>,
}

impl VfsTestSuite {
    pub(super) fn new(
        src_base: PathBuf,
        expect_files_base: PathBuf,
        adversarials_base: Option<PathBuf>,
    ) -> Self {
        std::fs::create_dir_all(&expect_files_base).expect("failed_to_create_dir_all");
        adversarials_base.as_ref().map(|adversarials_base| {
            std::fs::create_dir_all(&adversarials_base).expect("failed_to_create_dir_all")
        });
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

    pub(super) fn adversarials_base(&self) -> Option<&Path> {
        self.adversarials_base.as_ref().map(|p| p as &Path)
    }
}

pub(super) fn vfs_test_suites() -> Vec<VfsTestSuite> {
    let env = HuskyLangDevPaths::new();
    let dir = env
        .cargo_manifest_dir()
        .map(|p| p.to_owned())
        .unwrap_or("temp".into());
    vec![
        VfsTestSuite::new(
            env.lang_dev_library_dir().to_owned(),
            dir.join("expect-files/library"),
            None,
        ),
        VfsTestSuite::new(
            env.lang_dev_examples_dir().to_owned(),
            dir.join("expect-files/examples"),
            Some(dir.join("adversarials/examples")),
        ),
        VfsTestSuite::new(
            env.lang_dev_registry_dir().to_owned(),
            dir.join("expect-files/registry"),
            Some(dir.join("adversarials/registry")),
        ),
    ]
}
