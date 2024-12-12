import os
import sys
import warnings
from contextlib import redirect_stderr
from io import StringIO

# Set environment variables to disable warnings
os.environ["TRANSFORMERS_NO_ADVISORY_WARNINGS"] = "true"
os.environ["TOKENIZERS_PARALLELISM"] = "false"

# Suppress all warnings
warnings.filterwarnings("ignore")

# Redirect stderr during imports
stderr = StringIO()
with redirect_stderr(stderr):
    import transformers
    transformers.logging.set_verbosity_error()
    
    import spacy
    import benepar
    from spacy.tokens import Doc, Span

import spacy
import benepar
from spacy.tokens import Doc, Span

# Download spacy model if not present
try:
    nlp = spacy.load("en_core_web_sm")
except OSError:
    print("Downloading en_core_web_sm...")
    spacy.cli.download("en_core_web_sm")
    nlp = spacy.load("en_core_web_sm")

# Download and add the benepar parser
benepar.download('benepar_en3')
if 'benepar' not in nlp.pipe_names:
    nlp.add_pipe('benepar', config={"model": "benepar_en3"})

# Parse a simple sentence
text = "I love Python."
doc = nlp(text)

# Get the first sentence's constituency parse
sent = next(doc.sents)
print("\nConstituents:")
for constituent in sent._.constituents:
    print(f"Constituent: '{constituent.text}'")
    print(f"  Labels: {constituent._.labels}\n")

# Print the parse tree
print("\nParse Tree:")
print(sent._.parse_string)
import spacy
import benepar
from spacy.tokens import Doc, Span
import torch


# Download spacy model if not present
try:
    nlp = spacy.load("en_core_web_sm")
except OSError:
    print("Downloading en_core_web_sm...")
    spacy.cli.download("en_core_web_sm")
    nlp = spacy.load("en_core_web_sm")

# Ensure GPU is available
if not spacy.require_gpu():
    raise RuntimeError("GPU is required to run this script")

# Download and add the benepar parser
benepar.download('benepar_en3')
if 'benepar' not in nlp.pipe_names:
    nlp.add_pipe('benepar', config={"model": "benepar_en3"})

# Parse a simple sentence
text = "Let FORMULA1 be a topological space."
doc = nlp(text)

# Get the first sentence's constituency parse
sent = next(doc.sents)

# Print the parse tree
print("\nParse Tree:")
print(sent._.parse_string)
print(type(sent._.parse_string))
print(dir(sent._))
print(type(sent._.constituents))
print(sent._.constituents)

print("\nTokens in the sentence:")
for token in sent:
    print(f"Token: '{token.text}', POS: {token.pos_}, Dependency: {token.dep_}")

print("\nConstituents:")
for constituent in sent._.constituents:
    print(f"Type of constituent: {type(constituent)}")
    print(f"Constituent: '{constituent.text}'")  # The text content
    print(f"  Labels: {constituent._.labels}")   # Constituent labels (e.g., NP, VP, S)
    print(f"  Start: {constituent.start}")       # Start token index
    print(f"  End: {constituent.end}")          # End token index
    print(f"  Root: {constituent.root}")        # Root token of the span
    print(f"  Parent: {constituent._.parent}")   # Parent constituent
    print(f"  Children: {list(constituent._.children)}") # Child constituents