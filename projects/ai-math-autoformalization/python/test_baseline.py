import argparse
from prompts import PromptType, PromptGenerator
from api import ChatCompletionAPI
from utils import parse_testcase, parse_response
from lean_sandbox import LeanSandbox
import os
from datetime import datetime
import time
import json

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
    
    parser.add_argument('--wait-time', type=int, default=10,
                        help="waiting time for rate limit")

    return parser.parse_args()


def main():
    args = setup_args()
    
    # Setup directories
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    OUTPUT_DIR = f"{args.output_dir}/{args.model_name}_{args.prompt_type}_{timestamp}"
    os.makedirs(OUTPUT_DIR, exist_ok=True)

    # Add after OUTPUT_DIR setup:
    summary_file = f"{OUTPUT_DIR}/summary.json"
    summary_data = {
        "model_name": args.model_name,
        "prompt_type": args.prompt_type,
        "timestamp": timestamp,
        "results": {},
        "overall_stats": {
            "total_successes": 0,
            "total_attempts": 0,
            "success_rate": 0
        }
    }

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

    init_test_flag = 0

    for file in files:
        # number of successful attempts for this file
        file_successes = 0
        
        summary_data["results"][file] = {
            "system_message": "",  # Will be populated on first try
            "user_message": "",    # Will be populated on first try
            "attempts": []
        }
        
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
            
            if init_test_flag == 0:
                print(f'system_message\n {system_message}\n')
                print(f'user_message\n {user_message}\n')
                init_test_flag = 1
            
            if try_idx == 0:
                summary_data["results"][file]["system_message"] = system_message
                summary_data["results"][file]["user_message"] = user_message

            # Get completion and extract Lean code
            completion = api.chat_completion(messages, use_cache=False)
            lean_code = parse_response(completion["content"])
            
            # Save and test the generated code
            output_file = f"{OUTPUT_DIR}/{file.replace('.md', f'_try{try_idx}.lean')}"
            sandbox.generate_lean_checking_file(lean_code, output_file)

            exec_result = sandbox.run_lean_file(output_file)
            bug_msg = exec_result.stdout[:1000]
            is_success = bug_msg.strip() == "Success!"

            # Update summary data
            attempt_data = {
                "try_number": try_idx + 1,
                "output": bug_msg,
                "success": 1 if is_success else 0
            }
            summary_data["results"][file]["attempts"].append(attempt_data)
            
            # Update overall stats
            summary_data["overall_stats"]["total_attempts"] += 1
            if is_success:
                summary_data["overall_stats"]["total_successes"] += 1
                file_successes += 1
                total_successes += 1
            
            # Calculate and update success rate
            success_rate = (summary_data["overall_stats"]["total_successes"] / 
                          summary_data["overall_stats"]["total_attempts"]) * 100
            summary_data["overall_stats"]["success_rate"] = round(success_rate, 2)

            # Save updated summary after each attempt
            with open(summary_file, 'w') as f:
                json.dump(summary_data, f, indent=2)

            print('\n' + '='*50 + f' Try {try_idx + 1}, file: {file} ' + '='*50 + '\n')
            print(bug_msg)
            
            total_attempts += 1
            
            print(f'Waiting for {args.wait_time} seconds... due to rate limit')
            time.sleep(args.wait_time)
        
        # Add summary for this file
        summary_data["results"][file]["final_success_rate"] = f"{file_successes}/{args.max_tries}"

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