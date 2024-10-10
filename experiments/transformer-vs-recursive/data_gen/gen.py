import json
import gzip
import os
import pdb
from pathlib import Path

from data_gen.utils import rnd_codes_parallel

def write_data(dataset_filepath, data):
    compressed_filepath = f"{dataset_filepath}.json.gz"
    with gzip.open(compressed_filepath, 'wt', encoding='utf-8') as file:
        json.dump(data, file)
    print(f"Data written and compressed to {compressed_filepath}")

def main():
    dir_path = Path(os.path.join(os.environ["DATA_ROOT"],
                                 "mini-husky/basic"))
    os.makedirs(dir_path, exist_ok=True)

    params = [
        (100000, 10, 3, 0.2, 0.5),
        (100000, 20, 3, 0.2, 0.5),
        (100000, 20, 5, 0.2, 0.5)
    ]

    files_to_keep = []

    for n, max_fns, min_dist, use_var_rate, error_rate in params:
        dataset_filename = dir_path / f"dataset-n{n}-f{max_fns}-d{min_dist}-v{use_var_rate:.2f}-e{error_rate:.2f}"
        files_to_keep.append(f"{dataset_filename}.json.gz")

        # Generate the random codes
        data = rnd_codes_parallel(n, max_fns, min_dist, use_var_rate, error_rate, workers=16)

        print(data[0]["text"])

        # Write and compress the file
        write_data(dataset_filename, data)

    # Clear other files in the folder
    for entry in dir_path.iterdir():
        if entry.is_file() and entry.suffixes == ['.json', '.gz'] and str(entry) not in files_to_keep:
            entry.unlink()
            print(f"Removed old file: {entry}")

if __name__ == "__main__":
    main()
