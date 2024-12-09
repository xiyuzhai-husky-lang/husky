import os

EXAMPLES_DIR = "examples"

def examples():
    ret = ""
    idx = 1

    files = os.listdir(EXAMPLES_DIR)
    for file in files:
        if file.endswith(".md"):
            with open(f"{EXAMPLES_DIR}/{file}", "r") as f:
                ret += "### Example " + str(idx) + "\n"
                ret += f.read() + "\n"
                idx += 1
    return ret

SYSTEM_MESSAGE = f"You are a helpful assistant to write Lean 4 proofs. You will be given the statement of a problem, its proof in LaTeX, and a partial proof in Lean 4. Your task is to complete the Lean 4 proof by filling the 'sorry's and correct all mistakes given by bug messages. You can use any Lean 4 tactics and theorems from mathlib. Here are some examples for you to follow. \n{examples()}"

def prompt(problem, latex, lean, bug_msg = None):
    ret = f"Problem:\n{problem}\n\nLaTeX Proof:\n{latex}\n\nLean Proof:\n``` Lean\n{lean}\n```"

    if bug_msg:
        ret += f"\n\nBug Message:\n{bug_msg}"
    
    return ret
