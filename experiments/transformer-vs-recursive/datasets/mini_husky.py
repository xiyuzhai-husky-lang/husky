import os
from typing import List, Tuple, Optional, Dict
from pprint import pprint
import torch
from torch.utils.data import Dataset
from collections import Counter
import msgpack


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

    def _load_dataset(self) -> List[Tuple[List[str], List[Dict[str, Optional[str]]]]]:
        # Increase the tolerance for error rate matching
        tolerance = 1.0e-2  # Changed from 1.0e-4 to 1.0e-2

        for filename in os.listdir(self.data_dir):
            if filename.startswith("dataset-") and filename.endswith(".msgpack"):
                parts = filename[8:-8].split("-")
                file_n = int(parts[0][1:])
                file_max_fns = int(parts[1][1:])
                file_error_rate = float(parts[2][1:])

                if (
                    file_n == self.n
                    and file_max_fns == self.max_fns
                    and abs(file_error_rate - self.error_rate) <= tolerance
                ):

                    filepath = os.path.join(self.data_dir, filename)
                    print(
                        f"Loading dataset from {filepath}"
                    )  # Add this line for debugging
                    with open(filepath, "rb") as f:
                        return self._decode_rnd_codes(f.read())

        # List all available msgpack files
        all_msgpack_files = [
            f for f in os.listdir(self.data_dir) if f.endswith(".msgpack")
        ]
        available_files = "\n".join(all_msgpack_files)
        raise ValueError(
            f"Dataset with n={self.n}, max_fns={self.max_fns}, and error_rate "
            f"within {tolerance:.1e} of {self.error_rate:.4f} not found.\n\n"
            f"Available files:\n{available_files}\n\n"
        )

    def _decode_rnd_codes(
        self, packed_data: bytes
    ) -> List[Tuple[List[str], List[Dict[str, Optional[str]]]]]:
        unpacked_data = msgpack.unpackb(packed_data, raw=False)

        decoded_data = []
        for code_pair in unpacked_data:
            tokens, token_infos = code_pair

            decoded_token_infos = []
            for ast_kind, symbol_resolution, error in token_infos:
                decoded_info = {
                    "ast_kind": ast_kind,
                    "symbol_resolution": symbol_resolution,
                    "error": error,
                }
                decoded_token_infos.append(decoded_info)

            decoded_data.append((tokens, decoded_token_infos))

        return decoded_data

    def _build_vocabulary(self):
        word_counts = Counter()
        for words, _ in self.data:
            word_counts.update(words)
        return ["<PAD>", "<UNK>"] + [word for word, _ in word_counts.most_common()]

    def get_dataset(self) -> List[Tuple[List[str], List[Dict[str, Optional[str]]]]]:
        return self.data

    def __len__(self):
        return len(self.data)

    def __getitem__(self, idx):
        words, token_infos = self.data[idx]
        word_indices = torch.tensor(
            [self._word_to_index(word) for word in words], dtype=torch.long
        )
        return word_indices, token_infos

    def get_words(self, idx):
        return self.data[idx][0]

    def _word_to_index(self, word):
        return self.word_to_index.get(word, 1)  # 1 is the index for <UNK>


# Example usage
if __name__ == "__main__":
    # Load a specific dataset
    dataset = MiniHuskyDataset(100000, 20, 0.50)
    print(f"Dataset with 100000 samples, max_fns=20, error_rate=0.10:")
    print("First sample:")

    word_indices, token_infos = dataset[0]
    words = dataset.get_words(0)
    print(f"Sample 1:")
    print(f"  Words: {' '.join(words)}")
    print(f"  Word indices: {word_indices.tolist()}")
    print("  Token infos:")
    for i, info in enumerate(token_infos):
        print(f"    Token {i}: {info}")

    print(f"\nTotal samples: {len(dataset)}")

    # Print vocabulary
    print("\nVocabulary:")
    pprint(dataset.vocab[:10])  # Print first 10 vocabulary items
    print(f"Vocabulary size: {len(dataset.vocab)}")
