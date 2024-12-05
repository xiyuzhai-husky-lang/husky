.PHONY: run setup

# Define the conda environment name
CONDA_ENV_NAME := llm

# Ensure the conda environment is activated before running
run:
	conda run -n $(CONDA_ENV_NAME) python main.py

# Setup the conda environment and install dependencies
setup:
	conda create -n $(CONDA_ENV_NAME) python=3.11 -y
	conda run -n $(CONDA_ENV_NAME) pip install openai python-dotenv

# Optional: clean up environment
clean:
	conda env remove -n $(CONDA_ENV_NAME)
