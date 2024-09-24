#!/usr/bin/env python
from scholarly import scholarly

# Search for the paper on Google Scholar
query = scholarly.search_pubs(
    "A density-based algorithm for discovering clusters in large spatial databases with noise"
)

# Get the first paper from the search results
pub = next(query)

# Retrieve and print the BibTeX entry
bibtex_entry = scholarly.bibtex(pub)
print(bibtex_entry)
