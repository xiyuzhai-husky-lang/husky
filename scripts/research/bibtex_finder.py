#!/usr/bin/env python

"""
This script searches for BibTeX entries for a list of paper titles using the Google Scholar API (via the scholarly library) and merges the results into an existing BibTeX file. The script works as follows:

1. It reads a file containing paper titles (one title per line).
2. It reads an existing BibTeX file (if it exists) and extracts the titles from it.
3. For each paper title in the input file, it checks if the title is already present in the existing BibTeX file.
4. If the title is not found in the existing file, it sends a search request to Google Scholar for the BibTeX entry.
5. The script introduces a random delay between 5 and 10 seconds between each search request to avoid being rate-limited.
6. Finally, it merges the new BibTeX entries with the existing ones, ensuring no duplicates, and writes the result back to the specified BibTeX file.

Usage:
    ./bibtex_collector.py <paper_titles_file_path> -o <bib_file_path>

Example:
    ./bibtex_collector.py paper_titles.txt -o output.bib

Parameters:
- <paper_titles_file_path>: The path to a text file containing a list of paper titles (one per line).
- <bib_file_path>: The output BibTeX file where the results will be saved or merged with existing entries.

Requirements:
- scholarly library: You must have the 'scholarly' library installed (pip install scholarly).
"""

import sys
import os
import re
import time
import random
from typing import List, Optional, Set
from scholarly import scholarly


def search_bibtex(paper_title: str) -> Optional[str]:
    """Search for a paper on Google Scholar and return the BibTeX entry."""
    try:
        query = scholarly.search_pubs(paper_title)
        pub = next(query)
        return scholarly.bibtex(pub)
    except Exception as e:
        print(f"Error retrieving BibTeX for '{paper_title}': {str(e)}")
        return None


def read_paper_titles(file_path: str) -> List[str]:
    """Read paper titles from a file and return a list of titles."""
    try:
        with open(file_path, "r") as file:
            titles: List[str] = [line.strip() for line in file.readlines()]
        return titles
    except Exception as e:
        print(f"Error reading titles from file '{file_path}': {str(e)}")
        sys.exit(1)


def read_existing_bib_file(bib_file_path: str) -> List[str]:
    """Read existing BibTeX entries from a file and return them as a list of entries."""
    if not os.path.exists(bib_file_path):
        print(
            f"Warning: BibTeX file '{bib_file_path}' does not exist. A new file will be created."
        )
        return []
    try:
        with open(bib_file_path, "r") as bib_file:
            content: str = bib_file.read().strip()
            return content.split("\n\n") if content else []
    except Exception as e:
        print(f"Error reading BibTeX file '{bib_file_path}': {str(e)}")
        sys.exit(1)


def extract_bibtex_key(entry: str) -> Optional[str]:
    """Extract the unique key from a BibTeX entry (e.g., @article{key,...})."""
    match = re.search(r"@\w+\{([^,]+),", entry)
    return match.group(1) if match else None


def extract_bibtex_title(entry: str) -> Optional[str]:
    """Extract the title from a BibTeX entry."""
    match = re.search(r"title\s*=\s*\{([^}]+)\}", entry, re.IGNORECASE)
    return match.group(1) if match else None


def write_bibtex_to_file(bib_entries: List[str], output_file_path: str) -> None:
    """Write BibTeX entries to a file."""
    try:
        with open(output_file_path, "w") as bib_file:
            for entry in bib_entries:
                bib_file.write(entry + "\n\n")
        print(f"BibTeX entries merged and saved to '{output_file_path}'")
    except Exception as e:
        print(f"Error writing to BibTeX file '{output_file_path}': {str(e)}")
        sys.exit(1)


def main() -> None:
    """Main function to handle the command-line arguments and process the BibTeX collection."""
    if len(sys.argv) != 4 or sys.argv[2] != "-o":
        print("Usage: <script> <paper_titles_file_path> -o <bib_file_path>")
        sys.exit(1)

    paper_titles_file: str = sys.argv[1]
    bib_file_path: str = sys.argv[3]

    # Ensure that the paper titles file exists
    if not os.path.exists(paper_titles_file):
        print(f"Error: The paper titles file '{paper_titles_file}' does not exist.")
        sys.exit(1)

    # Collect BibTeX entries
    paper_titles: List[str] = read_paper_titles(paper_titles_file)

    # Read existing BibTeX entries from the output file (if it exists)
    existing_bib_entries: List[str] = read_existing_bib_file(bib_file_path)

    # Extract titles from the existing BibTeX entries
    existing_titles: Set[str] = {
        extract_bibtex_title(entry)
        for entry in existing_bib_entries
        if extract_bibtex_title(entry)
    }

    new_bib_entries: List[str] = []

    for title in paper_titles:
        # Check if the title is already in the existing BibTeX file, skip if found
        if title not in existing_titles:
            bibtex_entry: Optional[str] = search_bibtex(title)
            if bibtex_entry:
                new_bib_entries.append(bibtex_entry)

            # Introduce a random delay of at least 5 seconds between searches
            delay: float = random.uniform(5, 10)
            print(f"Sleeping for {delay:.2f} seconds to avoid rate limiting.")
            time.sleep(delay)
        else:
            print(f"Title '{title}' already found in existing BibTeX, skipping search.")

    if not new_bib_entries:
        print("No new BibTeX entries were retrieved.")
        sys.exit(0)

    # Create a set of keys for deduplication
    existing_keys: Set[str] = {
        extract_bibtex_key(entry)
        for entry in existing_bib_entries
        if extract_bibtex_key(entry)
    }

    # Filter out new entries that already exist based on their keys
    filtered_new_entries: List[str] = [
        entry
        for entry in new_bib_entries
        if extract_bibtex_key(entry) not in existing_keys
    ]

    if not filtered_new_entries:
        print("No new unique BibTeX entries to add.")
        sys.exit(0)

    # Merge new and existing entries, avoiding duplicates
    merged_bib_entries: List[str] = existing_bib_entries + filtered_new_entries

    # Write the merged BibTeX entries back to the output file
    write_bibtex_to_file(merged_bib_entries, bib_file_path)


if __name__ == "__main__":
    main()
