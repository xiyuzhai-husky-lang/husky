import argparse
from prompts import PromptType, PromptGenerator
from api import ChatCompletionAPI
from utils import parse_testcase, parse_response
from lean_sandbox import LeanSandbox
import os
from datetime import datetime

def setup_args():
    parser = argparse.ArgumentParser(description='Test LLM accuracy in generating Lean proofs')
    parser.add_argument('--model-type', type=str, default='gemini',
                       choices=['gemini', 'openai', 'local'],
                       help='LLM model to use')
    
    parser.add_argument('--model-name', type=str, default='gemini-1.5-flash',
                       help='Specific model name/version')
    
    parser.add_argument('--max-tries', type=int, default=5,
                       help='Number of attempts per problem')
    
    parser.add_argument('--prompt-type', type=str, default='only_problem',
                       choices=['only_problem', 'problem_latexproof'],
                       help='Type of prompt to use')
    
    parser.add_argument('--test-dir', type=str, default='testcases',
                       help='Directory containing test cases')
    
    parser.add_argument('--examples-dir', type=str, default='examples',
                       help='Directory containing examples')
    
    parser.add_argument('--output-dir', type=str, default='outputs',
                       help='Directory to save outputs, default is outputs_<timestamp>')
    
    parser.add_argument('--n-shot', type=int, default=2,
                       help='Number of examples to use')
    
    return parser.parse_args()


def main():
    args = setup_args()
    
    # Setup directories
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    OUTPUT_DIR = f"outputs_{timestamp}"
    os.makedirs(OUTPUT_DIR, exist_ok=True)

    # Initialize components
    sandbox = LeanSandbox()
    sandbox.setup_lean_project()
    
    api = ChatCompletionAPI(type=args.model_type, model=args.model_name)
    
    prompt_generator = PromptGenerator(
        n_shot=args.n_shot,
        examples_dir=args.examples_dir
    )

    # Get test files
    files = os.listdir(args.test_dir)
    files = [f for f in files if f.endswith('.md')]
    # files = ['batch_example5.md'] # debug
    
    print(f'Total test cases: {len(files)}, files: {files}')
    
    # Track statistics
    total_successes = 0
    total_attempts = 0
    results = {}

    prompt_type = PromptType[args.prompt_type.upper()]

    for file in files:
        file_successes = 0
        
        with open(f"{args.test_dir}/{file}", "r") as f:
            problem, latex, _ = parse_testcase(f.read())
        
        print(f'\n\nTesting file: {file}')
        print('=' * 50)
        
        for try_idx in range(args.max_tries):
            # Generate prompt based on type
            system_message, user_message = prompt_generator.generate_prompt(
                prompt_type=prompt_type,
                problem=problem,
                latex_proof=latex if prompt_type != PromptType.ONLY_PROBLEM else None
            )
            
            messages = [
                {"role": "system", "content": system_message},
                {"role": "user", "content": user_message},
            ]

            print(f'system_message\n {system_message}\n')
            print(f'user_message\n {user_message}\n')

            # Get completion and extract Lean code
            completion = api.chat_completion(messages, use_cache=False)
            lean_code = parse_response(completion["content"])
            
            # Save and test the generated code
            output_file = f"{OUTPUT_DIR}/{file.replace('.md', f'_try{try_idx}.lean')}"
            sandbox.generate_lean_checking_file(lean_code, output_file)

            exec_result = sandbox.run_lean_file(output_file)
            bug_msg = exec_result.stdout[:1000]

            print('\n' + '='*50 + f' Try {try_idx + 1}, file: {file} ' + '='*50 + '\n')
            print(bug_msg)
            
            total_attempts += 1
            if bug_msg.strip() == "Success!":
                file_successes += 1
                total_successes += 1
        
        results[file] = f"{file_successes}/{args.max_tries}"
        print(f'\nResults for {file}: {results[file]} successful attempts')

    # Print final statistics
    print('\n' + '=' * 50)
    print('Final Results:')
    for file, result in results.items():
        print(f'{file}: {result}')
    
    success_rate = (total_successes / total_attempts) * 100
    print(f'\nOverall success rate: {total_successes}/{total_attempts} ({success_rate:.2f}%)')

if __name__ == "__main__":
    main()