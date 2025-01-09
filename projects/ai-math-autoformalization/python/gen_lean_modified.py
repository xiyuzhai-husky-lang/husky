from prompts import PromptType, PromptGenerator
from api import ChatCompletionAPI
from utils import parse_testcase, parse_response
from lean_sandbox import LeanSandbox
import os

TESTCASES_DIR = "testcases"
EXAMPLES_DIR = "examples"
OUTPUT_DIR = "outputs"
os.makedirs(OUTPUT_DIR, exist_ok=True)


# Initialize LeanSandbox
sandbox = LeanSandbox()



# api = ChatCompletionAPI("local")
# api = ChatCompletionAPI("openai", model="gpt4o")
api = ChatCompletionAPI("gemini", model="gemini-1.5-flash")
max_tries = 10

# files = os.listdir(TESTCASES_DIR)
files = ['batch_example5.md']

print(f'total files: {len(files)}, containing: {files}')

# # Setup lean project using the sandbox
sandbox.setup_lean_project()

# Initialize PromptGenerator
prompt_generator = PromptGenerator(
    n_shot=2,
    examples_dir=EXAMPLES_DIR
)

for file in files:
    if file.endswith(".md"):

        with open(f"{TESTCASES_DIR}/{file}", "r") as f:
            problem, latex, lean = parse_testcase(f.read())
        
        bug_msg = None
        for _ in range(max_tries):
            # Use PromptGenerator to generate messages
            system_message, user_message = prompt_generator.generate_prompt(
                prompt_type=PromptType.PROBLEM_LATEXPROOF_LEANPROOF_BUG,
                problem=problem,
                latex_proof=latex,
                lean_proof=lean,
                bug_msg=bug_msg
            )
            
            # print(f'system_message\n {system_message}\n')
            # print(f'user_message\n {user_message}\n')
            
            messages = [
                {"role": "system", "content": system_message},
                {"role": "user", "content": user_message},
            ]

            # print(messages[1]["content"])

            completion = api.chat_completion(messages, use_cache=False)
            lean = parse_response(completion["content"])
            
            output_file = f"{OUTPUT_DIR}/{file.replace('.md', '.lean')}"
            sandbox.generate_lean_checking_file(lean, output_file)

            exec_result = sandbox.run_lean_file(output_file)
            bug_msg = exec_result.stdout[:1000]

            print(f'\n\n=================== try: {_}, file: {file} ===================\n\n')
            print(bug_msg)
            
            if bug_msg.strip() == "Success!":
                break