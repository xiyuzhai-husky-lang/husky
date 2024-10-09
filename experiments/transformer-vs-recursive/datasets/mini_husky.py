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
        dataset_path,
        desired_key = None
    ):
        self.max_len = 0
        self.header, self.data, self.stats = self._decode_rnd_codes(dataset_path)
        self.max_values = self.stats.max_values  # Add this line
        self.vocab = self._build_vocabulary()
        self.word_to_index = {word: i for i, word in enumerate(self.vocab)}

        if desired_key is not None:
            idx = self.header.index(desired_key)
            self.header = self.header[idx]
            self.data = [(words, token_infos[idx]) for words, token_infos in self.data]
            self.max_values = {desired_key: self.max_values[desired_key]}

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
            self.max_len = max(self.max_len, len(tokens))
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

    def get_max_len(self) -> int:
        return self.max_len

    def get_stats(self) -> DatasetStats:
        return self.stats

    def get_output_dims(self):
        return tuple(self.max_values[k] + 1 for k in self.header)


# Example usage
if __name__ == "__main__":
    # Load a specific dataset
    dataset = MiniHuskyDataset(
        os.path.join(os.environ["DATA_ROOT"],
                     "mini-husky/basic",
                     "dataset-n100000-f10-d3-v0.20-e0.50.msgpack"),
        desired_key="expected_type"
    )
    
    print("Max length:", dataset.get_max_len())

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
