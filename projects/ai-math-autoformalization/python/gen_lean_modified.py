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


# Initialize project if needed
def setup_lean_project():
    try:
        print("Initializing Lean project...")
        subprocess.run(["lake", "init", "mathproof"], check=False)
        
        print("Fetching mathlib...")
        subprocess.run(["lake", "update"], check=True)
        
    except subprocess.CalledProcessError as e:
        print(f"Error setting up project: {e}")
        raise

def run_lean_file(file_path):
    result = subprocess.run(
        ["lake", "env", "lean", "--run", file_path],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    
    # # Combine stdout and stderr for error checking
    # output = result.stdout + result.stderr
    
    # # Check for common error patterns
    # if "unknown module prefix 'Mathlib'" in output:
    #     print("Mathlib not found. Attempting to set up project...")
    #     setup_lean_project()
    #     # Try running the file again
    #     result = subprocess.run(
    #         ["lake", "env", "lean", "--run", file_path],
    #         stdout=subprocess.PIPE,
    #         stderr=subprocess.PIPE,
    #         text=True
    #     )
    #     output = result.stdout + result.stderr
    
    # return output

    return result


# api = ChatCompletionAPI("local")
# api = ChatCompletionAPI("openai", model="gpt4o")
api = ChatCompletionAPI("gemini", model="gemini-1.5-flash")
max_tries = 2

# files = os.listdir(TESTCASES_DIR)
files = ['batch_example9.md']

print(f'total files: {len(files)}, containing: {files}')

# Try to setup lean project first
# setup_lean_project()


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

            exec_result = run_lean_file(output_file)
            bug_msg = exec_result.stdout[:1000]

            print(f'\n\n=================== try: {_}, file: {file} ===================\n\n')
            print(bug_msg)
            
            if bug_msg.strip() == "Success!":
                break