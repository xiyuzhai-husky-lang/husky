import os
from typing import List, Tuple, Optional, Dict, NamedTuple
from pprint import pprint
import torch
from torch.utils.data import Dataset
from collections import Counter
import msgpack
from tqdm import tqdm

import pdb


class DatasetStats(NamedTuple):
    max_values: Dict[str, int]
    counters: Dict[str, Counter]
    percents: Dict[str, Dict[int, float]]


class MiniHuskyDataset(Dataset):
    def __init__(
        self,
        n: int,
        max_fns: int,
        min_dist: int,
        use_var_rate: float,
        error_rate: float,
        data_dir: str = "../../data/mini-husky/basic",
    ):
        self.data_dir = data_dir
        self.n = n
        self.max_fns = max_fns
        self.min_dist = min_dist
        self.use_var_rate = use_var_rate
        self.error_rate = error_rate
        self.header, self.data, self.stats = self._load_dataset()
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
                if len(parts) != 5:
                    continue
                file_n = int(parts[0][1:])
                file_max_fns = int(parts[1][1:])
                file_min_dist = int(parts[2][1:])
                file_use_var_rate = float(parts[3][1:])
                file_error_rate = float(parts[4][1:])

                if (
                    file_n == self.n
                    and file_max_fns == self.max_fns
                    and file_min_dist == self.min_dist
                    and abs(file_use_var_rate - self.use_var_rate) <= tolerance
                    and abs(file_error_rate - self.error_rate) <= tolerance
                ):

                    filepath = os.path.join(self.data_dir, filename)
                    print(f"Load dataset from {filepath}")
                    return self._decode_rnd_codes(filepath)

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
        self, filepath: str
    ) -> Tuple[
        List[str],
        List[Tuple[List[str], Tuple[List[int], List[int], List[int]]]], DatasetStats
    ]:
        header, data = None, None
        with open(filepath, "rb") as f:
            unpacker = msgpack.Unpacker(f)
            
            for unpacked in unpacker:
                if header is None:
                    header = unpacked
                elif data is None:
                    data = unpacked
                else:
                    print("Extra data found:", unpacked)
                    break

        decoded_data = []
        max_values = {k: 0 for k in header}
        counters = {k: Counter() for k in header}
        percents = {k: {} for k in header}

        for tokens, token_infos in tqdm(data):
            # Use list comprehension to unpack values efficiently
            fields = list(zip(*token_infos))
            
            for k, v in zip(header, fields):
                max_values[k] = max(max_values[k], max(v))
                counters[k].update(v)
            
            # Append the unpacked and decoded token infos
            decoded_data.append((tokens, tuple(fields)))

        for k in header:
            tot = sum(counters[k].values())
            percents[k] = {kk: vv / tot * 100 for kk, vv in counters[k].items()}
        
        stats = DatasetStats(
            max_values=max_values,
            counters=counters,
            percents=percents,
        )

        return header, decoded_data, stats

    def _build_vocabulary(self):
        word_counts = Counter()
        for words, _ in self.data:
            word_counts.update(words)
        return ["<PAD>", "<UNK>"] + [word for word, _ in word_counts.most_common()]

    def get_dataset(
        self,
    ) -> List[Tuple[List[str], Tuple]]:
        return self.data

    def __len__(self):
        return len(self.data)

    def __getitem__(self, idx):
        words, token_infos = self.data[idx]
        word_indices = torch.tensor(
            [self._word_to_index(word) for word in words], dtype=torch.long
        )
        return word_indices, tuple(torch.tensor(t, dtype=torch.long) for t in token_infos)

    def get_words(self, idx):
        return self.data[idx][0]

    def _word_to_index(self, word):
        return self.word_to_index.get(word, 1)

    def get_max_values(self) -> Dict[str, int]:
        return self.max_values

    def get_stats(self) -> DatasetStats:
        return self.stats

    def get_output_dims(self):
        return tuple(self.max_values[k] + 1 for k in self.header)


# Example usage
if __name__ == "__main__":
    # Load a specific dataset
    dataset = MiniHuskyDataset(
        n=100000,
        max_fns=100,
        min_dist=20,
        use_var_rate=0.2,
        error_rate=0.5,
        data_dir=os.path.join(os.environ["DATA_ROOT"], "mini-husky/basic")
    )

    # Print the output of __getitem__
    print("\n__getitem__ example:")
    idx = 0  # You can change this to any valid index
    word_indices, token_infos = dataset[idx]
    print(f"Sample {idx}:")
    print(f"  Word indices: {word_indices}")
    for k, v in zip(dataset.header, token_infos):
        print(f"  {k}: {v}")

    print(f"\nTotal samples: {len(dataset)}")

    # Print vocabulary
    print("\nVocabulary:")
    pprint(dataset.vocab[:50])
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
            return f"{obj: .2f}"
        else:
            return obj

    formatted_stats = format_stats(stats._asdict())
    pprint(formatted_stats, width=100)
