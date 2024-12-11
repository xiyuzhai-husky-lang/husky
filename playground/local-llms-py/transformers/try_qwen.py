from transformers import AutoModelForCausalLM, AutoTokenizer
import time

model_name = "Qwen/Qwen2.5-Math-7B-Instruct"
device = "cuda" # the device to load the model onto

model = AutoModelForCausalLM.from_pretrained(
    model_name,
    torch_dtype="auto",
    device_map="auto"
)
tokenizer = AutoTokenizer.from_pretrained(model_name)

# Long-form mathematical and analytical prompt
prompt = """Please solve the following complex problems:

1. Solve the differential equation: dy/dx + 2y = x^2 where y(0) = 1

2. Find the eigenvalues and eigenvectors of the matrix:
   [3 -2  4]
   [1  0  2]
   [2 -2  3]

3. Prove that the sequence an = (1 + 1/n)^n converges to e as n approaches infinity.

4. A factory produces widgets with a defect rate that follows a normal distribution with mean μ=2% and standard deviation σ=0.5%. 
   What is the probability that a randomly selected batch has a defect rate greater than 3%?

Please provide detailed step-by-step solutions for each problem."""

# Create a longer conversation with multiple turns
messages = [
    {"role": "system", "content": "You are a mathematical expert. Please provide detailed solutions with step-by-step reasoning, relevant formulas, and clear explanations. Include LaTeX formatting where appropriate."},
    {"role": "user", "content": prompt},
    {"role": "assistant", "content": "I'll help solve these problems step by step. Let's start with the first one..."},
    {"role": "user", "content": "Please also include numerical approximations where applicable and explain any assumptions made."},
]

text = tokenizer.apply_chat_template(
    messages,
    tokenize=False,
    add_generation_prompt=True
)
model_inputs = tokenizer([text], return_tensors="pt").to(device)

# Benchmark with multiple iterations
num_iterations = 3
total_tokens = 2048  # Increased token count for longer response
timing_results = []

print(f"Running {num_iterations} iterations with {total_tokens} tokens each...\n")

for i in range(num_iterations):
    start_time = time.time()
    
    generated_ids = model.generate(
        **model_inputs,
        max_new_tokens=total_tokens,
        temperature=0.7,  # Add some randomness
        top_p=0.9,
        do_sample=True
    )
    generated_ids = [
        output_ids[len(input_ids):] for input_ids, output_ids in zip(model_inputs.input_ids, generated_ids)
    ]
    
    end_time = time.time()
    generation_time = end_time - start_time
    timing_results.append(generation_time)
    
    response = tokenizer.batch_decode(generated_ids, skip_special_tokens=True)[0]
    
    print(f"\nIteration {i+1}:")
    print(f"Generation time: {generation_time:.2f} seconds")
    print(f"Tokens per second: {total_tokens/generation_time:.2f}")
    print(f"Response length (chars): {len(response)}")
    print("-" * 50)
    print(f"Response preview (first 500 chars):\n{response[:500]}...")
    print("-" * 50)

# Print summary statistics
avg_time = sum(timing_results) / len(timing_results)
print(f"\nPerformance Summary:")
print(f"Average generation time: {avg_time:.2f} seconds")
print(f"Average tokens per second: {total_tokens/avg_time:.2f}")
print(f"Fastest run: {min(timing_results):.2f} seconds")
print(f"Slowest run: {max(timing_results):.2f} seconds")