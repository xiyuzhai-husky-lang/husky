import warnings
warnings.filterwarnings("ignore", category=FutureWarning)
warnings.filterwarnings("ignore", category=UserWarning)
warnings.filterwarnings("ignore", module="transformers.*")
warnings.filterwarnings("ignore", module="torch.*")

import os
import time
import torch
os.environ["TOKENIZERS_PARALLELISM"] = "false"
os.environ["TRANSFORMERS_VERBOSITY"] = "error"

from spacy.cli import download
import spacy
import benepar

class Parser:
    def __init__(self, model_name="en_core_web_md", benepar_model='benepar_en3', device='cuda' if torch.cuda.is_available() else 'cpu'):
        self.device = device
        self.model_name = model_name
        self.benepar_model = benepar_model
        # Download models if not present
        try:
            self.nlp = spacy.load(model_name)
        except OSError:
            print(f"Downloading spacy model {model_name}...")
            download(model_name)
            self.nlp = spacy.load(model_name)
        
        if 'benepar' not in self.nlp.pipe_names:
            self.nlp.add_pipe('benepar', config={'model': benepar_model})

    def parse_batch(self, sentences, batch_size=32):
        """Batch process multiple sentences for better performance"""
        return [sent._.parse_string for doc in self.nlp.pipe(sentences, batch_size=batch_size) 
                for sent in doc.sents]
def run_benchmark(model_name, benepar_model, sentences, batch_size=32):
    parser = Parser(model_name, benepar_model)
    start_time = time.time()
    parse_trees = parser.parse_batch(sentences, batch_size=batch_size)
    elapsed_time = time.time() - start_time
    return elapsed_time


test_cases = {
    "short": [
        "The cat sleeps.",
        "Birds fly south.",
        "She reads books.",
        "Time flies fast.",
        "Dogs bark loudly."
    ] * 200,  # 1000 total sentences
    
    "medium": [
        "If the dog barks loudly, the startled cat will quickly jump over the wooden fence.",
        "The talented musician performed a beautiful symphony at the concert hall last night.",
        "Scientists have discovered a new species of butterfly in the Amazon rainforest.",
        "The old car broke down on the highway during our road trip to California.",
        "Students in the advanced class are preparing for their final examinations next week."
    ] * 200,
    
    "long": [
        "The experienced professor, who had been teaching advanced quantum mechanics at the prestigious university for more than two decades, carefully explained the complex theoretical concepts to his eager graduate students.",
        "Despite the challenging economic conditions and increasing competition in the global marketplace, the innovative technology company managed to significantly improve its quarterly financial performance through strategic investments and operational efficiency improvements.",
        "The renowned marine biologist spent several years studying the intricate relationships between various species of coral reef fish and their surrounding ecosystem, documenting previously unknown behavioral patterns that could have significant implications for ocean conservation efforts.",
        "The ambitious urban development project, which aims to transform the abandoned industrial district into a sustainable mixed-use neighborhood with affordable housing, green spaces, and modern infrastructure, has received widespread support from local community groups and environmental organizations.",
        "The international research team, comprising scientists from twelve different countries and multiple disciplines, has published groundbreaking findings about the potential impact of climate change on global agricultural systems and food security in the coming decades."
    ] * 200
}

if __name__ == "__main__":
    # ... [previous test sentence definitions remain the same] ...

    models = ["en_core_web_md"]  # We'll just use md since we proved size doesn't matter
    benepar_models = ["benepar_en3", "benepar_en3_large"]
    
    print(f"\nRunning benchmarks on device: {'CUDA' if torch.cuda.is_available() else 'CPU'}")
    if torch.cuda.is_available():
        print(f"GPU: {torch.cuda.get_device_name()}")
    
    results = {}
    
    for benepar_model in benepar_models:
        model_key = f"md_{benepar_model}"
        print(f"\n{'-'*50}")
        print(f"Testing {model_key}")
        results[model_key] = {}
        
        for length, sents in test_cases.items():
            elapsed_time = run_benchmark("en_core_web_md", benepar_model, sents)
            sentences_per_second = len(sents) / elapsed_time
            ms_per_sentence = (elapsed_time / len(sents)) * 1000
            
            results[model_key][length] = {
                'total_time': elapsed_time,
                'ms_per_sentence': ms_per_sentence,
                'sentences_per_second': sentences_per_second
            }
            
            print(f"\n{length.capitalize()} sentences:")
            print(f"Total time for {len(sents)} sentences: {elapsed_time:.2f} seconds")
            print(f"Average time per sentence: {ms_per_sentence:.2f} ms")
            print(f"Sentences per second: {sentences_per_second:.1f}")

    # Print comparison summary
    print("\n" + "="*50)
    print("SUMMARY (ms per sentence)")
    print("="*50)
    print(f"{'Model':<20} {'Short':>10} {'Medium':>10} {'Long':>10}")
    print("-"*50)
    for model_key in results:
        print(f"{model_key:.<20} {results[model_key]['short']['ms_per_sentence']:>10.2f} {results[model_key]['medium']['ms_per_sentence']:>10.2f} {results[model_key]['long']['ms_per_sentence']:>10.2f}")