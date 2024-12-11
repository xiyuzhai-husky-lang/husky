import warnings
warnings.filterwarnings("ignore", category=FutureWarning)
warnings.filterwarnings("ignore", category=UserWarning)
warnings.filterwarnings("ignore", module="transformers.*")
warnings.filterwarnings("ignore", module="torch.*")

import os
import torch
os.environ["TOKENIZERS_PARALLELISM"] = "false"
os.environ["TRANSFORMERS_VERBOSITY"] = "error"

from spacy.cli import download
import spacy
import benepar

class Parser:
    def __init__(self, device='cuda' if torch.cuda.is_available() else 'cpu'):
        self.device = device
        # Download models if not present
        try:
            self.nlp = spacy.load("en_core_web_md")
        except OSError:
            print("Downloading spacy model...")
            download("en_core_web_md")
            self.nlp = spacy.load("en_core_web_md")
        
        if 'benepar' not in self.nlp.pipe_names:
            # benepar.download('benepar_en3')
            self.nlp.add_pipe('benepar', config={'model': 'benepar_en3'})
        
        # spaCy handles GPU automatically

    def parse_sentence(self, sentence):
        doc = self.nlp(sentence)
        sent = list(doc.sents)[0]
        return sent._.parse_string
    
    def parse_batch(self, sentences, batch_size=32):
        """Batch process multiple sentences for better performance"""
        return [sent._.parse_string for doc in self.nlp.pipe(sentences, batch_size=batch_size) 
                for sent in doc.sents]

# Create parser once
parser = Parser()

def parse_sentence(sentence):
    return parser.parse_sentence(sentence)

# Example usage
if __name__ == "__main__":
    import cProfile
    import time

    # Example of batch processing
    sentences = ["If the dog barks, the cat will meow."] * 100

    # Time the batch processing
    start_time = time.time()
    parse_trees = parser.parse_batch(sentences)
    elapsed_time = time.time() - start_time
    
    print(f"Processed {len(sentences)} sentences in {elapsed_time:.2f} seconds")
    print(f"Average time per sentence: {(elapsed_time/len(sentences))*1000:.2f} ms")
