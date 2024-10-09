use crate::{
    config::{mayuri::MayuriConfig, nemu::NemuConfig},
    job::{MayuriJob, MayuriJobs},
    src::MayuriSrc,
    test::{MayuriTest, MayuriTests},
    *,
};
use gitignore::set_up_gitignore;
use husky_config_utils::IsConfig;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub struct MayuriFs {
    root: PathBuf,
    src: MayuriSrc,
    jobs: MayuriJobs,
    tests: MayuriTests,
    /// read from `<root>/Mayuri.toml`
    mayuri_config: MayuriConfig,
    /// read from `<root>/Nemu.toml`
    nemu_config: NemuConfig,
}

impl MayuriFs {
    pub fn new(root: PathBuf, append: bool) -> Self {
        set_up_gitignore(&root, append);
        let mayuri_config = match MayuriConfig::read_from_toml_file(&root.join("Mayuri.toml")) {
            Ok(config) => config,
            Err(e) => panic!("Failed to read Mayuri.toml in {:?}: {}", root, e),
        };

        let nemu_config = match NemuConfig::read_from_toml_file(&root.join("Nemu.toml")) {
            Ok(config) => config,
            Err(e) => panic!("Failed to read Nemu.toml in {:?}: {}", root, e),
        };

        let src = match MayuriSrc::new(root.join("src"), &["py"]) {
            Ok(src) => src,
            Err(e) => panic!("Failed to create MayuriSrc in {:?}: {}", root, e),
        };

        let jobs = MayuriJobs::from_dir(root.join("jobs"), &src);
        let tests = MayuriTests::from_dir(root.join("tests"), &src);

        MayuriFs {
            root,
            src,
            mayuri_config,
            nemu_config,
            jobs,
            tests,
        }
    }
}

#[test]
fn mayuri_fs_works() {
    use husky_path_utils::HuskyLangDevPaths;
    use std::env;
    use std::path::Path;

    let dev_paths = HuskyLangDevPaths::new();
    let root = dev_paths.root();

    // Change the current working directory to the root
    env::set_current_dir(&root).unwrap();

    // Use a relative path from the root
    let mayuri_prototype_dir = Path::new("experiments/mayuri-prototype");

    // Now the current directory is the root, so we use the relative path
    let fs = MayuriFs::new(mayuri_prototype_dir.to_path_buf(), false);
    expect_test::expect![[r#"
        MayuriFs {
            root: "experiments/mayuri-prototype",
            src: MayuriSrc {
                dir_path: "experiments/mayuri-prototype/src",
                shas: {
                    "datasets/ring.py": Sha512Output(`1b7409ccf0d5a34d3a77eaabfa9fe27427655be9297127ee9522aa1bf4046d4f945983678169cb1a7348edcac47ef0d9e2c924130e5bcc5f0d94937852c42f1b)`,
                    "main.py": Sha512Output(`7cbf0959df339401c90023f3938cd7d621927086356104ec66c9cc023ea263ab2102ea79abe6e6d24a693ca8c22cb429a8c9e65558b3d0613bf0613b31e9d061)`,
                    "datasets/gaussian.py": Sha512Output(`e50e95ebf2f1f1210d53a84753e7388f7f6ba1ea702ab19d1e983a82acdd20d87562d7ebe21c0e2b6f7f338969085497def15107c80182b39c26e4f516a61edb)`,
                    "visualize.py": Sha512Output(`967fef937b62d43ea3ef94de2c2b1c34b102563d41b281f6749b29660225de4f2df225137e54864544f59526bd77641ba40249f7ad60d5844001c9700331ea4e)`,
                    "models/fcn.py": Sha512Output(`1b7409ccf0d5a34d3a77eaabfa9fe27427655be9297127ee9522aa1bf4046d4f945983678169cb1a7348edcac47ef0d9e2c924130e5bcc5f0d94937852c42f1b)`,
                },
            },
            jobs: MayuriJobs {
                tests: [],
            },
            tests: MayuriTests {
                tests: [
                    MayuriTest {
                        path: "experiments/mayuri-prototype/tests/hello.yml",
                        rank: 0,
                        experiment: Experiment {
                            src: [
                                (
                                    ExperimentSrcDestinationPath {
                                        relative_path: "dataset.py",
                                    },
                                    ExperimentSrcOrigin {
                                        relative_path: "datasets/gaussian.py",
                                        content_sha: Sha512Output(`e50e95ebf2f1f1210d53a84753e7388f7f6ba1ea702ab19d1e983a82acdd20d87562d7ebe21c0e2b6f7f338969085497def15107c80182b39c26e4f516a61edb)`,
                                    },
                                ),
                                (
                                    ExperimentSrcDestinationPath {
                                        relative_path: "model.py",
                                    },
                                    ExperimentSrcOrigin {
                                        relative_path: "models/fcn.py",
                                        content_sha: Sha512Output(`1b7409ccf0d5a34d3a77eaabfa9fe27427655be9297127ee9522aa1bf4046d4f945983678169cb1a7348edcac47ef0d9e2c924130e5bcc5f0d94937852c42f1b)`,
                                    },
                                ),
                            ],
                            config: OrderedYaml("---
                            epochs: 10"),
                        },
                    },
                    MayuriTest {
                        path: "experiments/mayuri-prototype/tests/hello.yml",
                        rank: 1,
                        experiment: Experiment {
                            src: [
                                (
                                    ExperimentSrcDestinationPath {
                                        relative_path: "dataset.py",
                                    },
                                    ExperimentSrcOrigin {
                                        relative_path: "datasets/ring.py",
                                        content_sha: Sha512Output(`1b7409ccf0d5a34d3a77eaabfa9fe27427655be9297127ee9522aa1bf4046d4f945983678169cb1a7348edcac47ef0d9e2c924130e5bcc5f0d94937852c42f1b)`,
                                    },
                                ),
                                (
                                    ExperimentSrcDestinationPath {
                                        relative_path: "model.py",
                                    },
                                    ExperimentSrcOrigin {
                                        relative_path: "models/fcn.py",
                                        content_sha: Sha512Output(`1b7409ccf0d5a34d3a77eaabfa9fe27427655be9297127ee9522aa1bf4046d4f945983678169cb1a7348edcac47ef0d9e2c924130e5bcc5f0d94937852c42f1b)`,
                                    },
                                ),
                            ],
                            config: OrderedYaml("---
                            epochs: 10"),
                        },
                    },
                ],
            },
            mayuri_config: MayuriConfig,
            nemu_config: NemuConfig {
                src: [
                    SrcFragment {
                        path: "hello",
                    },
                ],
            },
        }
    "#]]
    .assert_debug_eq(&fs);
}
