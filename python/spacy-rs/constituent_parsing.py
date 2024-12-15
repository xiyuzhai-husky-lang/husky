import os
import warnings
from dataclasses import dataclass
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

    import spacy  # pyright: ignore
    import benepar  # pyright: ignore
    from spacy.tokens import Doc, Span

# Download spacy model if not present
try:
    nlp = spacy.load("en_core_web_sm")
except OSError:
    print("Downloading en_core_web_sm...")
    spacy.cli.download("en_core_web_sm")
    nlp = spacy.load("en_core_web_sm")

# # Download and add the benepar parser
# benepar.download('benepar_en3')
if "benepar" not in nlp.pipe_names:
    nlp.add_pipe("benepar", config={"model": "benepar_en3"})

# Ensure GPU is available
if not spacy.require_gpu():
    raise RuntimeError("GPU is required to run this script")

@dataclass
class Constituent:
    text: str
    span: Span
    children: list[Span]
    labels: list[str]

@dataclass
class ConstituentParsingOutput:
    tokens: list[str]
    constituents: list[Constituent]
    parse_string: str



def parse(text):
    doc = nlp(text)
    sent = next(doc.sents)
    return ConstituentParsingOutput(
        tokens=list(sent),
        constituents=[
            Constituent(
                text=constituent.text,
                span=constituent,
                children=list(constituent._.children),
                labels=list(constituent._.labels),
            )
            for constituent in sent._.constituents
        ],
        parse_string=sent._.parse_string,
    )


# # for testing purposes
# if __name__ == "__main__":
#     # Use the parse function
#     text = "Let FORMULA1 be a topological space."
#     output = parse(text)

#     # Print the results
#     print("\nParse Tree:")
#     print(output.parse_string)
#     print(type(output.parse_string))
#     print(dir(output.parse_string))
#     print(type(output.tokens))
#     print(output.tokens)

#     print("\nTokens in the sentence:")
#     for token in output.tokens:
#         print(f"Token: '{token.text}', POS: {token.pos_}, Dependency: {token.dep_}")
#     print("\nConstituents:")
#     for constituent in output.constituents:
#         print(f"Type of constituent: {type(constituent)}")
#         print(f"Constituent: '{constituent.text}'")  # The text content
#         print(
#             f"  Labels: {constituent._.labels}"
#         )  # Constituent labels (e.g., NP, VP, S)
#         print(f"  Start: {constituent.start}")  # Start token index
#         print(f"  End: {constituent.end}")  # End token index
#         print(f"  Root: {constituent.root}")  # Root token of the span
#         print(f"  Parent: {constituent._.parent}")  # Parent constituent
#         print(f"  Children: {list(constituent._.children)}")  # Child constituents
