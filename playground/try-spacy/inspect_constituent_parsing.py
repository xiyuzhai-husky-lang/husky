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

for constituent in sent._.constituents:
    print(f"Type of constituent: {type(constituent)}")
    print(f"Constituent: '{constituent.text}'")  # The text content
    print(f"  text: '{constituent.text}' (type: {type(constituent.text)})")
    print(f"  labels: {constituent._.labels} (type: {type(constituent._.labels)})")
    print(f"  start: {constituent.start} (type: {type(constituent.start)})")
    print(f"  end: {constituent.end} (type: {type(constituent.end)})")
    print(f"  root: {constituent.root} (type: {type(constituent.root)})")
    print(f"  parent: {constituent._.parent} (type: {type(constituent._.parent)})")
    print(f"  children: {list(constituent._.children)} (type: {type(constituent._.children)})")
    for child in constituent._.children:
        print(f"    child: {child} (type: {type(child)})")
        break
    break
