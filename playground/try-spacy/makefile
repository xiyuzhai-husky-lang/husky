.PHONY: env clean install run

env:
	pyenv install 3.12.4 --skip-existing && \
	pyenv local 3.12.4 && \
	python -m venv .venv

install-pyenv:# Install dependencies
	sudo apt-get update
	sudo apt-get install -y make build-essential libssl-dev zlib1g-dev \
	libbz2-dev libreadline-dev libsqlite3-dev wget curl llvm \
	libncursesw5-dev xz-utils tk-dev libxml2-dev libxmlsec1-dev libffi-dev liblzma-dev

	# Install pyenv
	curl https://pyenv.run | bash

install: env
	. .venv/bin/activate && \
	pip install --upgrade pip && \
	pip install spacy torch benepar

clean:
	rm -rf .venv
	find . -type f -name '*.pyc' -delete
	find . -type d -name '__pycache__' -delete

run-constituent-parsing:
	. .venv/bin/activate && python constituent_parsing.py
