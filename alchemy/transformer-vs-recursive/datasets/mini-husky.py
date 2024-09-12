import os
import random
from typing import List, Tuple


class MiniHuskyDataset:
    def __init__(self, data_dir: str = "data/mini-husky/basic"):
        self.data_dir = data_dir
        self.datasets = self._load_datasets()

    def _load_datasets(self) -> List[Tuple[int, int, float, List[Tuple[str, str]]]]:
        datasets = []
        for filename in os.listdir(self.data_dir):
            if filename.startswith("dataset-"):
                n, max_fns, error_rate = self._parse_filename(filename)
                file_path = os.path.join(self.data_dir, filename)
                with open(file_path, "r") as f:
                    data = [tuple(line.strip().split("\t")) for line in f]
                datasets.append((n, max_fns, error_rate, data))
        return datasets

    def _parse_filename(self, filename: str) -> Tuple[int, int, float]:
        parts = filename.split("-")
        n = int(parts[1][1:])
        max_fns = int(parts[2][1:])
        error_rate = float(parts[3][1:-4])
        return n, max_fns, error_rate

    def get_dataset(
        self, n: int, max_fns: int, error_rate: float
    ) -> List[Tuple[str, str]]:
        for dataset in self.datasets:
            if (
                dataset[0] == n
                and dataset[1] == max_fns
                and abs(dataset[2] - error_rate) < 1e-6
            ):
                return dataset[3]
        raise ValueError(
            f"Dataset with n={n}, max_fns={max_fns}, error_rate={error_rate} not found"
        )

    def get_random_dataset(self) -> Tuple[int, int, float, List[Tuple[str, str]]]:
        return random.choice(self.datasets)


# Example usage
if __name__ == "__main__":
    dataset = MiniHuskyDataset()

    # Get a specific dataset
    data = dataset.get_dataset(10000, 20, 0.10)
    print(f"Dataset with 10000 samples, max_fns=20, error_rate=0.10:")
    print(f"First 5 samples: {data[:5]}")
    print(f"Total samples: {len(data)}")

    # Get a random dataset
    n, max_fns, error_rate, data = dataset.get_random_dataset()
    print(f"\nRandom dataset with n={n}, max_fns={max_fns}, error_rate={error_rate}:")
    print(f"First 5 samples: {data[:5]}")
    print(f"Total samples: {len(data)}")
