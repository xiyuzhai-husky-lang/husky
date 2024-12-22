#! /bin/bash

set -e

source .venv/bin/activate
pip install --upgrade pip
pip install torch torchvision torchaudio
pip install "sglang[all]" --find-links https://flashinfer.ai/whl/cu124/torch2.4/flashinfer/
pip install -r requirements.txt

