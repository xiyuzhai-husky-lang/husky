from prompts import SYSTEM_MESSAGE, prompt
from api import ChatCompletionAPI
import os

TESTCASES_DIR = "testcases"
OUTPUT_DIR = "outputs"
os.makedirs(OUTPUT_DIR, exist_ok=True)

api = ChatCompletionAPI("local")

files = os.listdir(TESTCASES_DIR)
for file in files:
    if file.endswith(".md"):
        with open(f"{TESTCASES_DIR}/{file}", "r") as f:
            input = f.read()

        messages = [
            {"role": "system", "content": SYSTEM_MESSAGE},
            {"role": "user", "content": input},
        ]

        completion = api.chat_completion(messages)
        print(completion["content"])
        
        with open(f"{OUTPUT_DIR}/{file}", "w") as f:
            f.write(completion["content"])
