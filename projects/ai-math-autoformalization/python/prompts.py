import os

from enum import Enum
from typing import Optional, List
from dataclasses import dataclass
from utils import parse_testcase


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



class PromptType(Enum):
    ONLY_PROBLEM = "only_problem"
    PROBLEM_LATEXPROOF = "problem_latexproof"
    PROBLEM_LATEXPROOF_LEANPROOF_BUG = "problem_latexproof_leanproof_bug"

@dataclass
class Example:
    problem: str
    latex_proof: Optional[str] = None
    lean_proof: Optional[str] = None
    bug_msg: Optional[str] = None

    def format(self, prompt_type: PromptType, is_example: bool = False) -> str:
        """Format the example based on prompt type."""
        result = f"Problem:\n{self.problem}"
        
        if prompt_type in [PromptType.PROBLEM_LATEXPROOF, PromptType.PROBLEM_LATEXPROOF_LEANPROOF_BUG]:
            result += f"\n\nLaTeX Proof:\n{self.latex_proof}"
            
        # Always include Lean proof for examples, but follow prompt_type rules for current content
        if is_example or prompt_type == PromptType.PROBLEM_LATEXPROOF_LEANPROOF_BUG:
            result += f"\n\nLean Proof:\n``` Lean\n{self.lean_proof}\n```"
            if self.bug_msg:
                result += f"\n\nBug Message:\n{self.bug_msg}"
                
        return result

class PromptGenerator:
    def __init__(self, n_shot: int = 0, examples_dir: str = "examples"):
        self.n_shot = n_shot
        self.examples_dir = examples_dir
        self._system_messages = {
            PromptType.ONLY_PROBLEM: (
                "You are a helpful assistant to write Lean 4 proofs. You will be given a "
                "mathematical problem statement. Your task is to write a complete Lean 4 proof. "
                "You can use any Lean 4 tactics, and only allowed to use dependency from Mathlib."
                "Remember never use Lean 3 grammar, but use Lean 4 grammar!"
                "And you have to include the Lean proof inside the ``` Lean ``` block."
            ),
            PromptType.PROBLEM_LATEXPROOF: (
                "You are a helpful assistant to write Lean 4 proofs. You will be given the "
                "statement of a problem and its proof in LaTeX. Your task is to write a complete Lean 4 proof. "
                "You can use any Lean 4 tactics, and only allowed to use dependency from Mathlib."
                "Remember never use Lean 3 grammar, but use Lean 4 grammar!"
                "And you have to include the Lean proof inside the ``` Lean ``` block."
            ),
            PromptType.PROBLEM_LATEXPROOF_LEANPROOF_BUG: (
                "You are a helpful assistant to write Lean 4 proofs. You will be given the "
                "statement of a problem, its proof in LaTeX, and a partial proof in Lean 4. "
                "Your task is to complete the Lean 4 proof by filling the 'sorry's and correct "
                "all mistakes given by bug messages." 
                "You can use any Lean 4 tactics, and only allowed to use dependency from Mathlib."
                "Remember never use Lean 3 grammar, but use Lean 4 grammar!"
                "And you have to include the Lean proof inside the ``` Lean ``` block."
            )
        }

    def _get_examples(self) -> List[Example]:
        """Load examples from testcases directory."""
        examples = []
        files = os.listdir(self.examples_dir)
        
        for file in files[:self.n_shot]:
            if file.endswith(".md"):
                with open(f"{self.examples_dir}/{file}", "r") as f:
                    problem, latex_proof, lean_proof = parse_testcase(f.read())
                    examples.append(Example(
                        problem=problem,
                        latex_proof=latex_proof,
                        lean_proof=lean_proof
                    ))
        return examples


    def _validate_inputs(self, prompt_type: PromptType, problem: str, latex_proof: Optional[str], 
                        lean_proof: Optional[str], bug_msg: Optional[str]) -> None:
        """Validate inputs based on prompt type."""
        if prompt_type == PromptType.ONLY_PROBLEM:
            assert problem is not None and latex_proof is None and lean_proof is None and bug_msg is None
        elif prompt_type == PromptType.PROBLEM_LATEXPROOF:
            assert problem is not None and latex_proof is not None and lean_proof is None and bug_msg is None
        elif prompt_type == PromptType.PROBLEM_LATEXPROOF_LEANPROOF_BUG:
            assert problem is not None and latex_proof is not None and lean_proof is not None


    def generate_system_message(self, prompt_type: PromptType) -> str:
        """Generate system message based on prompt type and examples."""
        result = self._system_messages[prompt_type] + "\n\n"
        
        examples = self._get_examples()
        if examples:
            for i, example in enumerate(examples, 1):
                result += f"### Example {i}\n"
                result += example.format(prompt_type, is_example=True) + "\n\n"
        
        return result

    def generate_user_message(self, content: str) -> str:
        """Generate user prompt with the current task."""
        return f"### Current Task\n{content}\n\nLean Proof:"

    def generate_prompt(
        self,
        prompt_type: PromptType,
        problem: str,
        latex_proof: Optional[str] = None,
        lean_proof: Optional[str] = None,
        bug_msg: Optional[str] = None,
        if_strict: bool = False,
    ) -> tuple[str, str]:
        """Generate both system message and user prompt."""
        if if_strict:
            self._validate_inputs(prompt_type, problem, latex_proof, lean_proof, bug_msg)
            
        current_example = Example(
            problem=problem,
            latex_proof=latex_proof,
            lean_proof=lean_proof,
            bug_msg=bug_msg
        )
        
        content = current_example.format(prompt_type)
        system_message = self.generate_system_message(prompt_type)
        user_message = self.generate_user_message(content)
        
        return system_message, user_message
