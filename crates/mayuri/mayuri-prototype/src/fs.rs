mod test_subject;

use self::test_subject::MayuriTestSubject;
use husky_config_utils::IsConfig;
use serde::Deserialize;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use test_subject::MayuriTestSubjects;

#[derive(Debug)]
pub struct MayuriFs {
    root: PathBuf,
    test_subjects: MayuriTestSubjects,
    /// read from `<root>/Mayuri.toml`
    mayuri_config: MayuriConfig,
    /// read from `<root>/Nemu.toml`
    nemu_config: NemuConfig,
}

#[derive(Debug, Deserialize)]
pub struct MayuriConfig {
    // Add fields as needed
}

#[derive(Debug, Deserialize)]
pub struct NemuConfig {
    // Add fields as needed
}

impl MayuriFs {
    pub fn new(root: PathBuf) -> io::Result<Self> {
        let mayuri_config = MayuriConfig::read_from_toml_file(&root.join("Mayuri.toml"))?;
        let nemu_config = NemuConfig::read_from_toml_file(&root.join("Nemu.toml"))?;
        let test_subjects = MayuriTestSubjects::from_dir(root.join("tests"));
        Ok(MayuriFs {
            root,
            mayuri_config,
            nemu_config,
            test_subjects,
        })
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
    let fs = MayuriFs::new(mayuri_prototype_dir.to_path_buf()).unwrap();
    expect_test::expect![[r#"
        MayuriFs {
            root: "experiments/mayuri-prototype",
            test_subjects: MayuriTestSubjects {
                subjects: [
                    MayuriTestSubject {
                        path: "experiments/mayuri-prototype/tests/hello.yml",
                        rank: 0,
                        src: [
                            (
                                "dataset.py",
                                "datasets/gaussian.py",
                            ),
                            (
                                "model.py",
                                "models/fcn.py",
                            ),
                        ],
                        config: Hash(
                            {
                                String(
                                    "epochs",
                                ): Integer(
                                    10,
                                ),
                            },
                        ),
                    },
                    MayuriTestSubject {
                        path: "experiments/mayuri-prototype/tests/hello.yml",
                        rank: 1,
                        src: [
                            (
                                "dataset.py",
                                "datasets/ring.py",
                            ),
                            (
                                "model.py",
                                "models/fcn.py",
                            ),
                        ],
                        config: Hash(
                            {
                                String(
                                    "epochs",
                                ): Integer(
                                    10,
                                ),
                            },
                        ),
                    },
                ],
            },
            mayuri_config: MayuriConfig,
            nemu_config: NemuConfig,
        }
    "#]]
    .assert_debug_eq(&fs);
}
