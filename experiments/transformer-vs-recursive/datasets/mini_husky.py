import os
from typing import List, Tuple, Optional, Dict, NamedTuple
from pprint import pprint
import torch
from torch.utils.data import Dataset
from collections import Counter
import msgpack


class DatasetStats(NamedTuple):
    max_values: Dict[str, int]
    ast_kind_dist: Counter
    symbol_resolution_dist: Counter
    error_dist: Counter
    ast_kind_percent: Dict[int, float]
    symbol_resolution_percent: Dict[int, float]
    error_percent: Dict[int, float]


class MiniHuskyDataset(Dataset):
    def __init__(
        self,
        n: int,
        max_fns: int,
        error_rate: float,
        data_dir: str = "data/mini-husky/basic",
    ):
        self.data_dir = data_dir
        self.n = n
        self.max_fns = max_fns
        self.error_rate = error_rate
        self.data, self.stats = self._load_dataset()
        self.max_values = self.stats.max_values  # Add this line
        self.vocab = self._build_vocabulary()
        self.word_to_index = {word: i for i, word in enumerate(self.vocab)}

    def _load_dataset(
        self,
    ) -> Tuple[
        List[Tuple[List[str], Tuple[List[int], List[int], List[int]]]], DatasetStats
    ]:
        tolerance = 1.0e-2

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
                    print(f"Load dataset from {filepath}")
                    with open(filepath, "rb") as f:
                        return self._decode_rnd_codes(f.read())

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
    ) -> Tuple[
        List[Tuple[List[str], Tuple[List[int], List[int], List[int]]]], DatasetStats
    ]:
        unpacked_data = msgpack.unpackb(packed_data, raw=False)

        decoded_data = []
        max_values = {"ast_kind": 0, "symbol_resolution": 0, "error": 0}
        ast_kind_dist = Counter()
        symbol_resolution_dist = Counter()
        error_dist = Counter()

        for code_pair in unpacked_data:
            tokens, token_infos = code_pair

            ast_kinds = []
            symbol_resolutions = []
            errors = []
            for ast_kind, symbol_resolution, error in token_infos:
                ast_kinds.append(ast_kind)
                symbol_resolutions.append(symbol_resolution)
                errors.append(error)

                # Update max values and distributions
                max_values["ast_kind"] = max(max_values["ast_kind"], ast_kind)
                max_values["symbol_resolution"] = max(
                    max_values["symbol_resolution"], symbol_resolution
                )
                max_values["error"] = max(max_values["error"], error)

                ast_kind_dist[ast_kind] += 1
                symbol_resolution_dist[symbol_resolution] += 1
                error_dist[error] += 1

            decoded_token_infos = (ast_kinds, symbol_resolutions, errors)
            decoded_data.append((tokens, decoded_token_infos))

        # Calculate percentages
        total_tokens = sum(ast_kind_dist.values())
        ast_kind_percent = {k: v / total_tokens * 100 for k, v in ast_kind_dist.items()}
        symbol_resolution_percent = {
            k: v / total_tokens * 100 for k, v in symbol_resolution_dist.items()
        }
        error_percent = {k: v / total_tokens * 100 for k, v in error_dist.items()}

        stats = DatasetStats(
            max_values=max_values,
            ast_kind_dist=ast_kind_dist,
            symbol_resolution_dist=symbol_resolution_dist,
            error_dist=error_dist,
            ast_kind_percent=ast_kind_percent,
            symbol_resolution_percent=symbol_resolution_percent,
            error_percent=error_percent,
        )

        return decoded_data, stats

    def _build_vocabulary(self):
        word_counts = Counter()
        for words, _ in self.data:
            word_counts.update(words)
        return ["<PAD>", "<UNK>"] + [word for word, _ in word_counts.most_common()]

    def get_dataset(
        self,
    ) -> List[Tuple[List[str], Tuple[List[int], List[int], List[int]]]]:
        return self.data

    def __len__(self):
        return len(self.data)

    def __getitem__(self, idx):
        words, token_infos = self.data[idx]
        word_indices = torch.tensor(
            [self._word_to_index(word) for word in words], dtype=torch.long
        )
        ast_kinds, symbol_resolutions, errors = token_infos
        return word_indices, (
            torch.tensor(ast_kinds, dtype=torch.long),
            torch.tensor(symbol_resolutions, dtype=torch.long),
            torch.tensor(errors, dtype=torch.long),
        )

    def get_words(self, idx):
        return self.data[idx][0]

    def _word_to_index(self, word):
        return self.word_to_index.get(word, 1)  # 1 is the index for <UNK>

    def get_max_values(self) -> Dict[str, int]:
        return self.max_values

    def get_stats(self) -> DatasetStats:
        return self.stats

    def get_output_dims(self):
        return (
            self.max_values["ast_kind"] + 1,
            self.max_values["symbol_resolution"] + 1,
            self.max_values["error"] + 1,
        )


# Example usage
if __name__ == "__main__":
    # Load a specific dataset
    dataset = MiniHuskyDataset(100000, 20, 0.50)
    print(f"Dataset with 100000 samples, max_fns=20, error_rate=0.50:")

    # Print the output of __getitem__
    print("\n__getitem__ example:")
    idx = 0  # You can change this to any valid index
    word_indices, (ast_kinds, symbol_resolutions, errors) = dataset[idx]
    print(f"Sample {idx}:")
    print(f"  Word indices: {word_indices}")
    print(f"  AST kinds: {ast_kinds}")
    print(f"  Symbol resolutions: {symbol_resolutions}")
    print(f"  Errors: {errors}")

    print(f"\nTotal samples: {len(dataset)}")

    # Print vocabulary
    print("\nVocabulary:")
    pprint(dataset.vocab[:10])  # Print first 10 vocabulary items
    print(f"Vocabulary size: {len(dataset.vocab)}")

    # Print maximum values
    print("\nMaximum values:")
    pprint(dataset.get_max_values())

    # Print statistics
    print("\nStatistics:")
    stats = dataset.get_stats()

    def format_stats(obj):
        if isinstance(obj, dict):
            return {k: format_stats(v) for k, v in obj.items()}
        elif isinstance(obj, float):
            return f"{obj:.2f}"
        else:
            return obj

    formatted_stats = format_stats(stats._asdict())
    pprint(formatted_stats, width=100)
