```
conda create -n lean-env python=3.10 -y
conda activate lean-env

pip install requests  # For API calls
pip install openai  # For OpenAI API
pip install diskcache  # For caching
pip install -q -U google-generativeai  # For Gemini API

# Install Lean
curl https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh -sSf | sh

# Initialize Lean
source $HOME/.elan/env
elan toolchain
```

[Problems about mathlib](https://github.com/leanprover-community/mathlib4/wiki/Using-mathlib4-as-a-dependency)

