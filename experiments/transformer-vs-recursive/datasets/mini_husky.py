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
        filename_pattern = f"dataset-n{self.n}-f{self.max_fns}-e*.txt"
        matching_files = [
            f for f in os.listdir(self.data_dir) if f.startswith(filename_pattern[:-5])
        ]

        error_rate_threshold = 1e-4  # Much smaller threshold as requested

        close_enough_files = [
            f
            for f in matching_files
            if abs(float(f.split("-e")[-1][:-4]) - self.error_rate)
            <= error_rate_threshold
        ]

        if not close_enough_files:
            raise ValueError(
                f"Dataset with n={self.n}, max_fns={self.max_fns}, and error_rate "
                f"within {error_rate_threshold:.1e} of {self.error_rate:.4f} not found"
            )

        # Choose the file with the closest error rate
        closest_file = min(
            close_enough_files,
            key=lambda f: abs(float(f.split("-e")[-1][:-4]) - self.error_rate),
        )
        file_path = os.path.join(self.data_dir, closest_file)

        with open(file_path, "r") as f:
            lines = f.readlines()
            return [
                (
                    lines[i].strip().split(),
                    [int(num) for num in lines[i + 1].strip().split()],
                )
                for i in range(0, len(lines), 2)
            ]

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
        words, error_positions = self.data[idx]
        word_indices = torch.tensor(
            [self._word_to_index(word) for word in words], dtype=torch.long
        )
        error_tensor = torch.zeros(len(words), dtype=torch.float)
        error_tensor[error_positions] = 1.0
        return word_indices, error_tensor  # Only return word indices and error tensor

    def get_words(self, idx):
        return self.data[idx][0]  # Return only the words for the given index

    def _word_to_index(self, word):
        return self.word_to_index.get(word, 1)  # 1 is the index for <UNK>


# Example usage
if __name__ == "__main__":
    # Load a specific dataset
    dataset = MiniHuskyDataset(100000, 20, 0.10)
    print(f"Dataset with 100000 samples, max_fns=20, error_rate=0.10:")
    print("First sample with errors:")

    for i in range(len(dataset)):
        word_indices, error_tensor = dataset[i]
        words = dataset.get_words(i)  # Get words using the new method
        if error_tensor.sum() > 0:  # Find the first sample with errors
            print(f"Sample {i + 1}:")
            print(f"  Words: {' '.join(words)}")  # Print words in a single line
            print(f"  Error tensor: {error_tensor.tolist()}")
            print("  Words at error positions:")
            error_positions = error_tensor.nonzero().squeeze().tolist()
            for pos in error_positions:
                print(f"    Position {pos}: '{words[pos]}'")
            break  # Stop after finding the first sample with errors

    print(f"\nTotal samples: {len(dataset)}")

    # Print vocabulary
    print("\nVocabulary:")
    pprint(dataset.vocab[:10])  # Print first 10 vocabulary items
    print(f"Vocabulary size: {len(dataset.vocab)}")
