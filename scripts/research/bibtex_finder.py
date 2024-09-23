#!/usr/bin/env python
from scholarly import scholarly


def get_bibtex_for_title(title):
    # Search for the paper on Google Scholar
    search_query = scholarly.search_pubs(title)
    try:
        paper = next(search_query)
        # Fetch the BibTeX entry
        bibtex = paper["bibtex"]  # Access the 'bibtex' value using the key
        print(f"BibTeX for '{title}':\n{bibtex}")
    except StopIteration:
        print(f"No results found for '{title}'")


# Example usage
paper_title = "Attention is All You Need"
get_bibtex_for_title(paper_title)
