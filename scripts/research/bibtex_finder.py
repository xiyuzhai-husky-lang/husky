#!/usr/bin/env python

"""
This script searches for BibTeX entries for a list of paper titles using Google Scholar or Semantic Scholar, based on user preference.
The results are merged into an existing BibTeX file. The script can be configured to:
1. Use only Google Scholar.
2. Use only Semantic Scholar.
3. Try Google Scholar first, and if it fails, fall back to Semantic Scholar.

You can also configure the maximum interval for sleep between API requests.

Usage:
    ./bibtex_collector.py <paper_titles_file_path> -o <bib_file_path> --use <google|semantic|both> --max-sleep <max_interval>

Example:
    ./bibtex_collector.py paper_titles.txt -o output.bib --use google --max-sleep 10

Requirements:
- scholarly: Google Scholar library (pip install scholarly)
- requests: To use Semantic Scholar API (pip install requests)
"""

import sys
import os
import re
import time
import random
import requests
from typing import List, Optional, Set
from scholarly import scholarly

SEMANTIC_SCHOLAR_API = "https://api.semanticscholar.org/graph/v1/paper/search"


def search_bibtex_google(paper_title: str) -> Optional[str]:
    """Search for a paper on Google Scholar and return the BibTeX entry."""
    try:
        query = scholarly.search_pubs(paper_title)
        pub = next(query)
        return scholarly.bibtex(pub)
    except Exception as e:
        print(f"Google Scholar search failed for '{paper_title}': {str(e)}")
        return None


def search_bibtex_semantic_scholar(paper_title: str) -> Optional[str]:
    """Search for a paper on Semantic Scholar and return the BibTeX entry."""
    try:
        response = requests.get(
            SEMANTIC_SCHOLAR_API,
            params={"query": paper_title, "fields": "title,authors,venue,year,bibtex"},
            timeout=10,  # Timeout for requests
        )
        response.raise_for_status()  # Raises an HTTPError for bad responses
        data = response.json()

        # Check if the response contains any results
        if "data" in data and len(data["data"]) > 0:
            # Return the BibTeX entry if available
            return data["data"][0].get("bibtex")
        else:
            print(f"No results found for '{paper_title}' on Semantic Scholar.")
            return None
    except Exception as e:
        print(f"Semantic Scholar search failed for '{paper_title}': {str(e)}")
        return None


def search_bibtex(paper_title: str, search_engine: str) -> Optional[str]:
    """
    Search for a paper based on the user's search engine preference.

    Args:
        paper_title: The title of the paper to search for.
        search_engine: The search engine to use ('google', 'semantic', or 'both').

    Returns:
        The BibTeX entry if found, otherwise None.
    """
    if search_engine == "google":
        return search_bibtex_google(paper_title)

    elif search_engine == "semantic":
        return search_bibtex_semantic_scholar(paper_title)

    elif search_engine == "both":
        bibtex_entry = search_bibtex_google(paper_title)
        if not bibtex_entry:
            print(f"Falling back to Semantic Scholar for '{paper_title}'")
            bibtex_entry = search_bibtex_semantic_scholar(paper_title)
        return bibtex_entry

    else:
        print(f"Invalid search engine option: {search_engine}")
        sys.exit(1)


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
    if len(sys.argv) != 8 or sys.argv[4] != "--use" or sys.argv[6] != "--max-sleep":
        print(
            "Usage: <script> <paper_titles_file_path> -o <bib_file_path> --use <google|semantic|both> --max-sleep <max_interval>"
        )
        sys.exit(1)

    paper_titles_file: str = sys.argv[1]
    bib_file_path: str = sys.argv[3]
    search_engine: str = sys.argv[
        5
    ]  # Get the search engine preference (google, semantic, or both)
    max_sleep: int = int(sys.argv[7])  # Get the max sleep interval (user defined)

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
            bibtex_entry: Optional[str] = search_bibtex(title, search_engine)
            if not bibtex_entry:
                sys.exit(1)  # Exit on failure

            new_bib_entries.append(bibtex_entry)

            # Introduce a random delay between 0 seconds and the specified max_sleep interval
            delay: float = random.uniform(0, max_sleep)
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
