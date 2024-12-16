import sglang as sgl
import asyncio
import os
from multiprocessing import freeze_support

prompts = [
    "Hello, my name is",
    "The capital of France is",
    "The future of AI is",
]

sampling_params = {"temperature": 0.8, "top_p": 0.95}

async def run_async_main(llm):
    outputs = await llm.async_generate(prompts, sampling_params)
    for prompt, output in zip(prompts, outputs):
        print(f"\nPrompt: {prompt}")
        print(f"Generated text: {output['text']}")

def main():
    # Initialize the engine here, inside __main__ and after freeze_support
    llm = sgl.Engine(model_path=os.path.expandvars("$HOME/.llms/models/Qwen2-Math-7B-Instruct"))

    print("\n=== Testing asynchronous batch generation ===")
    asyncio.run(run_async_main(llm))
    llm.shutdown()

if __name__ == '__main__':
    freeze_support()
    main()
