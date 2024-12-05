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

files = os.listdir(TESTCASES_DIR)
for file in files:
    if file.endswith(".md"):

        with open(f"{TESTCASES_DIR}/{file}", "r") as f:
            problem, latex, lean = parse_testcase(f.read())
        
        bug_msg = None
        for _ in range(10):
            messages = [
                {"role": "system", "content": SYSTEM_MESSAGE},
                {"role": "user", "content": prompt(problem, latex, lean, bug_msg)},
            ]2

            print(messages[1]["content"])

            completion = api.chat_completion(messages)
            lean = parse_response(completion["content"])
            
            output_file = f"{OUTPUT_DIR}/{file.replace('.md', '.lean')}"
            with open(output_file, "w") as f:
                f.write(lean + "\n" + MAIN_FUNC)

            exec_result = subprocess.run(["lake", "env", "lean", "--run", output_file], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)

            bug_msg = exec_result.stdout[:500]

            if bug_msg.strip() == "Success!":
                break
