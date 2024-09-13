import os
from typing import List, Tuple
from pprint import pprint


class MiniHuskyDataset:
    def __init__(
        self,
        n: int,
        max_fns: int,
        error_rate: float,
        data_dir: str = "../../data/mini-husky/basic",
    ):
        self.data_dir = data_dir
        self.n = n
        self.max_fns = max_fns
        self.error_rate = error_rate
        self.data = self._load_dataset()

    def _load_dataset(self) -> List[Tuple[List[str], List[int]]]:
        filename = f"dataset-n{self.n}-f{self.max_fns}-e{self.error_rate:.2f}.txt"
        file_path = os.path.join(self.data_dir, filename)
        try:
            with open(file_path, "r") as f:
                lines = f.readlines()
                return [
                    (
                        self._parse_line(lines[i].strip())[0],
                        [int(num) for num in lines[i + 1].strip().split()],
                    )
                    for i in range(0, len(lines), 2)
                ]
        except FileNotFoundError:
            raise ValueError(
                f"Dataset with n={self.n}, max_fns={self.max_fns}, error_rate={self.error_rate} not found"
            )

    def _parse_line(self, line: str) -> Tuple[List[str], List[int]]:
        words = line.split()
        return (
            words,
            [],
        )  # Return empty list for numbers, will be filled in _load_dataset

    def get_dataset(self) -> List[Tuple[List[str], List[int]]]:
        return self.data


# Example usage
if __name__ == "__main__":
    # Load a specific dataset
    dataset = MiniHuskyDataset(100000, 20, 0.10)
    data = dataset.get_dataset()
    print(f"Dataset with 100000 samples, max_fns=20, error_rate=0.10:")
    print("First 5 samples:")
    pprint(data[:5], width=100, compact=True)
    print(f"Total samples: {len(data)}")
