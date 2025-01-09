from prompts import SYSTEM_MESSAGE, prompt
from api import ChatCompletionAPI
from utils import parse_testcase, parse_response
import os
import subprocess

TESTCASES_DIR = "testcases"
OUTPUT_DIR = "outputs"
os.makedirs(OUTPUT_DIR, exist_ok=True)

MAIN_FUNC = \
'''
def main : IO Unit :=
  IO.println "Success!"
'''

api = ChatCompletionAPI("local")
# api = ChatCompletionAPI("openai", model="gpt4o")
# api = ChatCompletionAPI("gemini", model="gemini-1.5-flash")


max_tries = 10

files = os.listdir(TESTCASES_DIR)
print(f'total files: {len(files)}, containing: {files}')

for file in files:
    if file.endswith(".md"):

        with open(f"{TESTCASES_DIR}/{file}", "r") as f:
            problem, latex, lean = parse_testcase(f.read())
        
        bug_msg = None
        for _ in range(max_tries):
            messages = [
                {"role": "system", "content": SYSTEM_MESSAGE},
                {"role": "user", "content": prompt(problem, latex, lean, bug_msg)},
            ]

            print(messages[1]["content"])

            completion = api.chat_completion(messages, use_cache=False)
            lean = parse_response(completion["content"])
            
            output_file = f"{OUTPUT_DIR}/{file.replace('.md', '.lean')}"
            with open(output_file, "w") as f:
                f.write(lean + "\n" + MAIN_FUNC)

            exec_result = subprocess.run(["lake", "env", "lean", "--run", output_file], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)

            bug_msg = exec_result.stdout[:500]
            print(f'\n\n=================== try: {_}, file: {file} ===================\n\n')
            print(bug_msg)
            
            if bug_msg.strip() == "Success!":
                break