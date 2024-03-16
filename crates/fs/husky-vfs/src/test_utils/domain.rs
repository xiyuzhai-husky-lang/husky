use super::*;

#[derive(Debug)]
#[salsa::derive_debug_with_db]
pub(super) struct TestDomain {
    src_base: PathBuf,
    expect_files_base: PathBuf,
    adversarials_base: Option<PathBuf>,
}

impl TestDomain {
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

pub struct TestDomainsConfig(TestDomainsConfigImpl);

pub enum TestDomainsConfigImpl {
    Full,
    ExcludeLibrary,
    ExamplesOnly,
}

/// # exports
impl TestDomainsConfig {
    pub const COMPTIME: Self = TestDomainsConfig(TestDomainsConfigImpl::ExcludeLibrary);
    pub const DEVTIME: Self = TestDomainsConfig(TestDomainsConfigImpl::ExcludeLibrary);
    pub const FS: Self = TestDomainsConfig(TestDomainsConfigImpl::Full);
    pub const HIR: Self = TestDomainsConfig(TestDomainsConfigImpl::Full);
    pub const IDE: Self = TestDomainsConfig(TestDomainsConfigImpl::Full);
    pub const LEX: Self = TestDomainsConfig(TestDomainsConfigImpl::Full);
    pub const LINKAGE: Self = TestDomainsConfig(TestDomainsConfigImpl::ExcludeLibrary);
    pub const LINKTIME: Self = TestDomainsConfig(TestDomainsConfigImpl::ExcludeLibrary);
    pub const KERNEL: Self = TestDomainsConfig(TestDomainsConfigImpl::Full);
    pub const SYNTAX: Self = TestDomainsConfig(TestDomainsConfigImpl::Full);
    pub const SEMANTICS: Self = TestDomainsConfig(TestDomainsConfigImpl::Full);
    pub const TOML: Self = TestDomainsConfig(TestDomainsConfigImpl::Full);
    pub const VAL: Self = TestDomainsConfig(TestDomainsConfigImpl::ExcludeLibrary);
}

impl TestDomainsConfig {
    pub(super) fn test_domains(&self) -> Vec<TestDomain> {
        let env = HuskyLangDevPaths::new();
        let dir = env
            .cargo_manifest_dir()
            .map(|p| p.to_owned())
            .unwrap_or("temp".into());
        match self.0 {
            TestDomainsConfigImpl::Full => {
                vec![
                    TestDomain::new(
                        env.lang_dev_library_dir().to_owned(),
                        dir.join("expect-files/library"),
                        None,
                    ),
                    TestDomain::new(
                        env.lang_dev_examples_dir().to_owned(),
                        dir.join("expect-files/examples"),
                        Some(dir.join("adversarials/examples")),
                    ),
                    TestDomain::new(
                        env.lang_dev_registry_dir().to_owned(),
                        dir.join("expect-files/registry"),
                        Some(dir.join("adversarials/registry")),
                    ),
                ]
            }
            TestDomainsConfigImpl::ExcludeLibrary => {
                vec![
                    TestDomain::new(
                        env.lang_dev_examples_dir().to_owned(),
                        dir.join("expect-files/examples"),
                        Some(dir.join("adversarials/examples")),
                    ),
                    TestDomain::new(
                        env.lang_dev_registry_dir().to_owned(),
                        dir.join("expect-files/registry"),
                        Some(dir.join("adversarials/registry")),
                    ),
                ]
            }
            TestDomainsConfigImpl::ExamplesOnly => vec![TestDomain::new(
                env.lang_dev_examples_dir().to_owned(),
                dir.join("expect-files/examples"),
                Some(dir.join("adversarials/examples")),
            )],
        }
    }
}
