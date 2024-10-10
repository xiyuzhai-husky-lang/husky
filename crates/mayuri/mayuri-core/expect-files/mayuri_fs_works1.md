```rust
MayuriFs {
    root: "experiments/mayuri-prototype",
    src: MayuriSrc {
        dir_path: "experiments/mayuri-prototype/src",
        files: {
            "datasets/ring.py": MayuriSrcFile {
                content: "",
            },
            "visualize.py": MayuriSrcFile {
                content: "from dataset import Dataset\nfrom model import train_model\n\nif __name__ == \"__main__\":\n    # Generate a 2D Gaussian dataset\n    mean = (0, 0)\n    cov = [[1, 0.5], [0.5, 1]]\n    data = generate_2d_gaussian(n_samples=2000, mean=mean, cov=cov)\n\n    # Create data loaders\n    train_loader, val_loader, test_loader = create_data_loaders(data, batch_size=32)\n\n    print(f\"Number of batches in train_loader: {len(train_loader)}\")\n    print(f\"Number of batches in val_loader: {len(val_loader)}\")\n    print(f\"Number of batches in test_loader: {len(test_loader)}\")\n\n    # Plot the dataset\n    plot_2d_gaussian(data, title=\"2D Gaussian Distribution Example\")\n",
            },
            "main.py": MayuriSrcFile {
                content: "print(\"Hello world\")\n",
            },
            "models/fcn.py": MayuriSrcFile {
                content: "",
            },
            "datasets/gaussian.py": MayuriSrcFile {
                content: "import numpy as np\nimport matplotlib.pyplot as plt\nimport torch\nfrom torch.utils.data import Dataset, DataLoader\nfrom sklearn.model_selection import train_test_split\n\n\nclass GaussianDataset(Dataset):\n    def __init__(self, n_samples=1000, mean=(0, 0), cov=[[1, 0], [0, 1]]):\n        self.data = torch.FloatTensor(generate_2d_gaussian(n_samples, mean, cov))\n\n    def __len__(self):\n        return len(self.data)\n\n    def __getitem__(self, idx):\n        return self.data[idx]\n\n\ndef generate_2d_gaussian(n_samples=1000, mean=(0, 0), cov=[[1, 0], [0, 1]]):\n    \"\"\"\n    Generate a 2D Gaussian distribution dataset.\n\n    Args:\n        n_samples (int): Number of samples to generate.\n        mean (tuple): Mean of the distribution (x, y).\n        cov (list): 2x2 covariance matrix.\n\n    Returns:\n        numpy.ndarray: Array of shape (n_samples, 2) containing the generated points.\n    \"\"\"\n    return np.random.multivariate_normal(mean, cov, n_samples)\n\n\ndef create_data_loaders(\n    data, batch_size=32, train_ratio=0.7, val_ratio=0.15, test_ratio=0.15\n):\n    \"\"\"\n    Create train, validation, and test data loaders.\n\n    Args:\n        data (numpy.ndarray): The dataset to split and load.\n        batch_size (int): Batch size for the data loaders.\n        train_ratio (float): Ratio of data to use for training.\n        val_ratio (float): Ratio of data to use for validation.\n        test_ratio (float): Ratio of data to use for testing.\n\n    Returns:\n        tuple: (train_loader, val_loader, test_loader)\n    \"\"\"\n    # Split the data into train+val and test\n    train_val, test = train_test_split(data, test_size=test_ratio, random_state=42)\n\n    # Split the train+val into train and val\n    train, val = train_test_split(\n        train_val, test_size=val_ratio / (train_ratio + val_ratio), random_state=42\n    )\n\n    # Create datasets\n    train_dataset = GaussianDataset(train)\n    val_dataset = GaussianDataset(val)\n    test_dataset = GaussianDataset(test)\n\n    # Create data loaders\n    train_loader = DataLoader(train_dataset, batch_size=batch_size, shuffle=True)\n    val_loader = DataLoader(val_dataset, batch_size=batch_size)\n    test_loader = DataLoader(test_dataset, batch_size=batch_size)\n\n    return train_loader, val_loader, test_loader\n\n\ndef plot_2d_gaussian(data, title=\"2D Gaussian Distribution\"):\n    \"\"\"\n    Plot the 2D Gaussian distribution.\n\n    Args:\n        data (numpy.ndarray): Array of shape (n_samples, 2) containing the data points.\n        title (str): Title for the plot.\n    \"\"\"\n    plt.figure(figsize=(10, 8))\n    plt.scatter(data[:, 0], data[:, 1], alpha=0.5)\n    plt.title(title)\n    plt.xlabel(\"X\")\n    plt.ylabel(\"Y\")\n    plt.grid(True)\n    plt.show()\n",
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
                    path: ExperimentPath {
                        src_paths: [
                            (
                                "dataset.py",
                                "datasets/gaussian.py",
                            ),
                            (
                                "main.py",
                                "main.py",
                            ),
                            (
                                "model.py",
                                "models/fcn.py",
                            ),
                        ],
                    },
                    path_sha256: Sha256Output(`054011e0734b59cad67bdd16650cba5663f72d23d304e92b77895c96dd0230bf`),
                    src_files: [
                        (
                            "dataset.py",
                            MayuriSrcFile {
                                content: "import numpy as np\nimport matplotlib.pyplot as plt\nimport torch\nfrom torch.utils.data import Dataset, DataLoader\nfrom sklearn.model_selection import train_test_split\n\n\nclass GaussianDataset(Dataset):\n    def __init__(self, n_samples=1000, mean=(0, 0), cov=[[1, 0], [0, 1]]):\n        self.data = torch.FloatTensor(generate_2d_gaussian(n_samples, mean, cov))\n\n    def __len__(self):\n        return len(self.data)\n\n    def __getitem__(self, idx):\n        return self.data[idx]\n\n\ndef generate_2d_gaussian(n_samples=1000, mean=(0, 0), cov=[[1, 0], [0, 1]]):\n    \"\"\"\n    Generate a 2D Gaussian distribution dataset.\n\n    Args:\n        n_samples (int): Number of samples to generate.\n        mean (tuple): Mean of the distribution (x, y).\n        cov (list): 2x2 covariance matrix.\n\n    Returns:\n        numpy.ndarray: Array of shape (n_samples, 2) containing the generated points.\n    \"\"\"\n    return np.random.multivariate_normal(mean, cov, n_samples)\n\n\ndef create_data_loaders(\n    data, batch_size=32, train_ratio=0.7, val_ratio=0.15, test_ratio=0.15\n):\n    \"\"\"\n    Create train, validation, and test data loaders.\n\n    Args:\n        data (numpy.ndarray): The dataset to split and load.\n        batch_size (int): Batch size for the data loaders.\n        train_ratio (float): Ratio of data to use for training.\n        val_ratio (float): Ratio of data to use for validation.\n        test_ratio (float): Ratio of data to use for testing.\n\n    Returns:\n        tuple: (train_loader, val_loader, test_loader)\n    \"\"\"\n    # Split the data into train+val and test\n    train_val, test = train_test_split(data, test_size=test_ratio, random_state=42)\n\n    # Split the train+val into train and val\n    train, val = train_test_split(\n        train_val, test_size=val_ratio / (train_ratio + val_ratio), random_state=42\n    )\n\n    # Create datasets\n    train_dataset = GaussianDataset(train)\n    val_dataset = GaussianDataset(val)\n    test_dataset = GaussianDataset(test)\n\n    # Create data loaders\n    train_loader = DataLoader(train_dataset, batch_size=batch_size, shuffle=True)\n    val_loader = DataLoader(val_dataset, batch_size=batch_size)\n    test_loader = DataLoader(test_dataset, batch_size=batch_size)\n\n    return train_loader, val_loader, test_loader\n\n\ndef plot_2d_gaussian(data, title=\"2D Gaussian Distribution\"):\n    \"\"\"\n    Plot the 2D Gaussian distribution.\n\n    Args:\n        data (numpy.ndarray): Array of shape (n_samples, 2) containing the data points.\n        title (str): Title for the plot.\n    \"\"\"\n    plt.figure(figsize=(10, 8))\n    plt.scatter(data[:, 0], data[:, 1], alpha=0.5)\n    plt.title(title)\n    plt.xlabel(\"X\")\n    plt.ylabel(\"Y\")\n    plt.grid(True)\n    plt.show()\n",
                            },
                        ),
                        (
                            "main.py",
                            MayuriSrcFile {
                                content: "print(\"Hello world\")\n",
                            },
                        ),
                        (
                            "model.py",
                            MayuriSrcFile {
                                content: "",
                            },
                        ),
                    ],
                    src_files_sha512: Sha512Output(`f0a3e98e8a5a236591afb6f7ae1294f9813a222de765e45f517ba6682a38ff1dcea7bc597aca00f6d648fe2f299c3a65cf4c457b442624be8d15e184a9635f28)`,
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
                    path: ExperimentPath {
                        src_paths: [
                            (
                                "dataset.py",
                                "datasets/ring.py",
                            ),
                            (
                                "main.py",
                                "main.py",
                            ),
                            (
                                "model.py",
                                "models/fcn.py",
                            ),
                        ],
                    },
                    path_sha256: Sha256Output(`27c6a8ec11464a35f3df4ded9951a1d3e4cb20e9003c254caee998b6cddf18f7`),
                    src_files: [
                        (
                            "dataset.py",
                            MayuriSrcFile {
                                content: "",
                            },
                        ),
                        (
                            "main.py",
                            MayuriSrcFile {
                                content: "print(\"Hello world\")\n",
                            },
                        ),
                        (
                            "model.py",
                            MayuriSrcFile {
                                content: "",
                            },
                        ),
                    ],
                    src_files_sha512: Sha512Output(`dcd48e2c23bf2a237ffbe8591d78dc979f0402e250c3dda7e5502fa1c9370bd7876c68868935d52939b645e9bb0c46681f35703063c5fa5f56651c9f38beb7b7)`,
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
        src_paths: [
            SrcPath {
                path: "main.py",
            },
        ],
    },
}
```