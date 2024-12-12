import spacy
import cupy
import thinc

# Enable GPU
spacy.require_gpu()
thinc.api.require_gpu()

# Check spaCy's GPU detection
print("Spacy GPU:", spacy.prefer_gpu())
print("Thinc GPU:", thinc.api.prefer_gpu())
print("CuPy CUDA:", cupy.cuda.runtime.getDeviceCount())