import os
from typing import List, Tuple
from pprint import pprint
import torch
from torch.utils.data import Dataset
from collections import Counter


class MiniHuskyDataset(Dataset):
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
        self.vocab = self._build_vocabulary()
        self.word_to_index = {word: i for i, word in enumerate(self.vocab)}

    def _load_dataset(self) -> List[Tuple[List[str], List[int]]]:
        filename = f"dataset-n{self.n}-f{self.max_fns}-e{self.error_rate:.2f}.txt"
        file_path = os.path.join(self.data_dir, filename)
        try:
            with open(file_path, "r") as f:
                lines = f.readlines()
                return [
                    (
                        lines[i].strip().split(),
                        [int(num) for num in lines[i + 1].strip().split()],
                    )
                    for i in range(0, len(lines), 2)
                ]
        except FileNotFoundError:
            raise ValueError(
                f"Dataset with n={self.n}, max_fns={self.max_fns}, error_rate={self.error_rate} not found"
            )

    def _build_vocabulary(self):
        word_counts = Counter()
        for words, _ in self.data:
            word_counts.update(words)
        return ["<PAD>", "<UNK>"] + [word for word, _ in word_counts.most_common()]

    def get_dataset(self) -> List[Tuple[List[str], List[int]]]:
        return self.data

    def __len__(self):
        return len(self.data)

    def __getitem__(self, idx):
        words, labels = self.data[idx]
        word_indices = torch.tensor(
            [self._word_to_index(word) for word in words], dtype=torch.long
        )
        labels = torch.tensor(labels, dtype=torch.long)
        return word_indices, labels

    def _word_to_index(self, word):
        return self.word_to_index.get(word, 1)  # 1 is the index for <UNK>


# Example usage
if __name__ == "__main__":
    # Load a specific dataset
    dataset = MiniHuskyDataset(100000, 20, 0.10)
    print(f"Dataset with 100000 samples, max_fns=20, error_rate=0.10:")
    print("First 5 samples:")
    for i in range(5):
        word_indices, labels = dataset[i]
        print(f"Sample {i + 1}:")
        print(f"  Word indices: {word_indices}")
        print(f"  Labels: {labels}")
    print(f"Total samples: {len(dataset)}")

    # Print vocabulary
    print("\nVocabulary:")
    pprint(dataset.vocab)
    print(f"Vocabulary size: {len(dataset.vocab)}")
