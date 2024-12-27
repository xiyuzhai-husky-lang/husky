import sglang as sgl
import asyncio
import os
from multiprocessing import freeze_support


def main():
    # Initialize the engine here, inside __main__ and after freeze_support
    llm = sgl.Engine(
        model_path=os.path.expandvars("$HOME/.llms/models/Qwen2-Math-7B-Instruct")
    )

    prompts = [
        "Hello, my name is",
        "The president of the United States is",
        "The capital of France is",
        "The future of AI is",
    ]

    sampling_params = {"temperature": 0.8, "top_p": 0.95}

    outputs = llm.generate(prompts, sampling_params)
    for prompt, output in zip(prompts, outputs):
        print("===============================")
        print(f"Prompt: {prompt}\nGenerated text: {output['text']}")
    llm.shutdown()


if __name__ == "__main__":
    freeze_support()
    main()
