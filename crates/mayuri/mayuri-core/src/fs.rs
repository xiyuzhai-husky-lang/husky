use crate::{
    config::{mayuri::MayuriConfig, nemu::NemuConfig},
    job::{MayuriJob, MayuriJobs},
    makefile::MayuriMakefileExtracted,
    src::MayuriSrc,
    test::{MayuriTest, MayuriTests},
    *,
};
use gitignore::set_up_gitignore;
use husky_config_utils::IsConfig;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct MayuriFs {
    root: PathBuf,
    src: MayuriSrc,
    makefile: MayuriMakefileExtracted,
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
        let makefile = MayuriMakefileExtracted::new(&root, ["run".to_string()])
            .unwrap_or_else(|e| panic!("Failed to extract Makefile rules: {}", e));
        let jobs = MayuriJobs::from_dir(root.join("jobs"), &src, &makefile, &nemu_config);
        let tests = MayuriTests::from_dir(root.join("tests"), &src, &makefile, &nemu_config);
        MayuriFs {
            root,
            makefile,
            src,
            mayuri_config,
            nemu_config,
            jobs,
            tests,
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn src(&self) -> &MayuriSrc {
        &self.src
    }

    pub fn makefile(&self) -> &MayuriMakefileExtracted {
        &self.makefile
    }

    pub fn jobs(&self) -> &MayuriJobs {
        &self.jobs
    }

    pub fn tests(&self) -> &MayuriTests {
        &self.tests
    }

    pub fn mayuri_config(&self) -> &MayuriConfig {
        &self.mayuri_config
    }

    pub fn nemu_config(&self) -> &NemuConfig {
        &self.nemu_config
    }

    pub fn run_all_tests(&self) {
        self.tests.run_all()
    }

    pub fn run_all_jobs(&self) {
        self.jobs.run_all()
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
                    "visualize.py": MayuriCode {
                        sha: Sha512Output(`967fef937b62d43ea3ef94de2c2b1c34b102563d41b281f6749b29660225de4f2df225137e54864544f59526bd77641ba40249f7ad60d5844001c9700331ea4e)`,
                        content: "from dataset import Dataset\nfrom model import train_model\n\nif __name__ == \"__main__\":\n    # Generate a 2D Gaussian dataset\n    mean = (0, 0)\n    cov = [[1, 0.5], [0.5, 1]]\n    data = generate_2d_gaussian(n_samples=2000, mean=mean, cov=cov)\n\n    # Create data loaders\n    train_loader, val_loader, test_loader = create_data_loaders(data, batch_size=32)\n\n    print(f\"Number of batches in train_loader: {len(train_loader)}\")\n    print(f\"Number of batches in val_loader: {len(val_loader)}\")\n    print(f\"Number of batches in test_loader: {len(test_loader)}\")\n\n    # Plot the dataset\n    plot_2d_gaussian(data, title=\"2D Gaussian Distribution Example\")\n",
                    },
                    "models/fcn.py": MayuriCode {
                        sha: Sha512Output(`1b7409ccf0d5a34d3a77eaabfa9fe27427655be9297127ee9522aa1bf4046d4f945983678169cb1a7348edcac47ef0d9e2c924130e5bcc5f0d94937852c42f1b)`,
                        content: "",
                    },
                    "datasets/ring.py": MayuriCode {
                        sha: Sha512Output(`1b7409ccf0d5a34d3a77eaabfa9fe27427655be9297127ee9522aa1bf4046d4f945983678169cb1a7348edcac47ef0d9e2c924130e5bcc5f0d94937852c42f1b)`,
                        content: "",
                    },
                    "datasets/gaussian.py": MayuriCode {
                        sha: Sha512Output(`e50e95ebf2f1f1210d53a84753e7388f7f6ba1ea702ab19d1e983a82acdd20d87562d7ebe21c0e2b6f7f338969085497def15107c80182b39c26e4f516a61edb)`,
                        content: "import numpy as np\nimport matplotlib.pyplot as plt\nimport torch\nfrom torch.utils.data import Dataset, DataLoader\nfrom sklearn.model_selection import train_test_split\n\n\nclass GaussianDataset(Dataset):\n    def __init__(self, n_samples=1000, mean=(0, 0), cov=[[1, 0], [0, 1]]):\n        self.data = torch.FloatTensor(generate_2d_gaussian(n_samples, mean, cov))\n\n    def __len__(self):\n        return len(self.data)\n\n    def __getitem__(self, idx):\n        return self.data[idx]\n\n\ndef generate_2d_gaussian(n_samples=1000, mean=(0, 0), cov=[[1, 0], [0, 1]]):\n    \"\"\"\n    Generate a 2D Gaussian distribution dataset.\n\n    Args:\n        n_samples (int): Number of samples to generate.\n        mean (tuple): Mean of the distribution (x, y).\n        cov (list): 2x2 covariance matrix.\n\n    Returns:\n        numpy.ndarray: Array of shape (n_samples, 2) containing the generated points.\n    \"\"\"\n    return np.random.multivariate_normal(mean, cov, n_samples)\n\n\ndef create_data_loaders(\n    data, batch_size=32, train_ratio=0.7, val_ratio=0.15, test_ratio=0.15\n):\n    \"\"\"\n    Create train, validation, and test data loaders.\n\n    Args:\n        data (numpy.ndarray): The dataset to split and load.\n        batch_size (int): Batch size for the data loaders.\n        train_ratio (float): Ratio of data to use for training.\n        val_ratio (float): Ratio of data to use for validation.\n        test_ratio (float): Ratio of data to use for testing.\n\n    Returns:\n        tuple: (train_loader, val_loader, test_loader)\n    \"\"\"\n    # Split the data into train+val and test\n    train_val, test = train_test_split(data, test_size=test_ratio, random_state=42)\n\n    # Split the train+val into train and val\n    train, val = train_test_split(\n        train_val, test_size=val_ratio / (train_ratio + val_ratio), random_state=42\n    )\n\n    # Create datasets\n    train_dataset = GaussianDataset(train)\n    val_dataset = GaussianDataset(val)\n    test_dataset = GaussianDataset(test)\n\n    # Create data loaders\n    train_loader = DataLoader(train_dataset, batch_size=batch_size, shuffle=True)\n    val_loader = DataLoader(val_dataset, batch_size=batch_size)\n    test_loader = DataLoader(test_dataset, batch_size=batch_size)\n\n    return train_loader, val_loader, test_loader\n\n\ndef plot_2d_gaussian(data, title=\"2D Gaussian Distribution\"):\n    \"\"\"\n    Plot the 2D Gaussian distribution.\n\n    Args:\n        data (numpy.ndarray): Array of shape (n_samples, 2) containing the data points.\n        title (str): Title for the plot.\n    \"\"\"\n    plt.figure(figsize=(10, 8))\n    plt.scatter(data[:, 0], data[:, 1], alpha=0.5)\n    plt.title(title)\n    plt.xlabel(\"X\")\n    plt.ylabel(\"Y\")\n    plt.grid(True)\n    plt.show()\n",
                    },
                    "main.py": MayuriCode {
                        sha: Sha512Output(`7cbf0959df339401c90023f3938cd7d621927086356104ec66c9cc023ea263ab2102ea79abe6e6d24a693ca8c22cb429a8c9e65558b3d0613bf0613b31e9d061)`,
                        content: "print(\"Hello world\")\n",
                    },
                },
            },
            makefile: MayuriMakefileExtracted {
                content: "run:\n\tpython main.py\n",
            },
            jobs: MayuriJobs {
                jobs: [],
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
                                        sha: Sha512Output(`e50e95ebf2f1f1210d53a84753e7388f7f6ba1ea702ab19d1e983a82acdd20d87562d7ebe21c0e2b6f7f338969085497def15107c80182b39c26e4f516a61edb)`,
                                    },
                                ),
                                (
                                    ExperimentSrcDestinationPath {
                                        relative_path: "model.py",
                                    },
                                    ExperimentSrcOrigin {
                                        relative_path: "models/fcn.py",
                                        sha: Sha512Output(`1b7409ccf0d5a34d3a77eaabfa9fe27427655be9297127ee9522aa1bf4046d4f945983678169cb1a7348edcac47ef0d9e2c924130e5bcc5f0d94937852c42f1b)`,
                                    },
                                ),
                            ],
                            config: OrderedYaml("---
                            epochs: 10"),
                            makefile: MayuriMakefileExtracted {
                                content: "run:\n\tpython main.py\n",
                            },
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
                                        sha: Sha512Output(`1b7409ccf0d5a34d3a77eaabfa9fe27427655be9297127ee9522aa1bf4046d4f945983678169cb1a7348edcac47ef0d9e2c924130e5bcc5f0d94937852c42f1b)`,
                                    },
                                ),
                                (
                                    ExperimentSrcDestinationPath {
                                        relative_path: "model.py",
                                    },
                                    ExperimentSrcOrigin {
                                        relative_path: "models/fcn.py",
                                        sha: Sha512Output(`1b7409ccf0d5a34d3a77eaabfa9fe27427655be9297127ee9522aa1bf4046d4f945983678169cb1a7348edcac47ef0d9e2c924130e5bcc5f0d94937852c42f1b)`,
                                    },
                                ),
                            ],
                            config: OrderedYaml("---
                            epochs: 10"),
                            makefile: MayuriMakefileExtracted {
                                content: "run:\n\tpython main.py\n",
                            },
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
    // fs.run_all_tests();
    // fs.run_all_jobs();
}
