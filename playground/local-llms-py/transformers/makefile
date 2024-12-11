.PHONY: env clean

# Environment name
ENV_NAME = transformers-env

# Create conda environment
create-env:
	conda create -n $(ENV_NAME) python=3.9 -y
	conda activate $(ENV_NAME) && \
	pip install transformers torch numpy pandas scikit-learn tqdm

try-qwen:
	python try_qwen.py

# Clean up environment 
clean:
	conda env remove -n $(ENV_NAME) -y