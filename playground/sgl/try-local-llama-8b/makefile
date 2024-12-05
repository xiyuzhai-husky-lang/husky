
# Define the conda environment name
CONDA_ENV_NAME := llama-env

run:
	conda run -n $(CONDA_ENV_NAME) python main.py

setup:
	conda create -n $(CONDA_ENV_NAME) python=3.10 -y
	conda run -n $(CONDA_ENV_NAME) pip install llama-cpp-python