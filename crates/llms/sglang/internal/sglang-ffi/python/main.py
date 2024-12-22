import sglang as sgl
import os

llm = sgl.Engine(model_path=os.path.expandvars("$HOME/.llms/models/Qwen2-7B-Instruct"))

sampling_params = {"temperature": 0.8, "top_p": 0.95}


def generate_text_batch(prompts: list[str]):
    return llm.generate(prompts, sampling_params)
