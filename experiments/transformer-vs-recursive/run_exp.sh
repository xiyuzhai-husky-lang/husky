#!/bin/bash

# Define an associative array where keys are GPUs and values are experiment type and seed pairs
# Format for each entry: gpu="exp:seed"
declare -A gpu_exp_seed_map=(
    [3]="transformer:142857"
    [4]="transformer:2225393"
    [2]="transformer:20000308"
    [7]="transformer:2018011309"
)

# Loop through the associative array
for gpu in "${!gpu_exp_seed_map[@]}"; do
    # Extract experiment type and seed using IFS (Internal Field Separator)
    IFS=':' read -r exp seed <<< "${gpu_exp_seed_map[$gpu]}"
    session_name="${exp}_gpu${gpu}_seed${seed}"
    
    # Check if the tmux session already exists and create it if it doesn't
    tmux has-session -t "$session_name" 2>/dev/null
    if [ $? != 0 ]; then
        # Use the experiment type in the command to run the appropriate script
        tmux new-session -d -s "$session_name" "CUDA_VISIBLE_DEVICES=$gpu python train_$exp.py --seed=$seed"
        echo "Experiment started in tmux session: $session_name"
    else
        echo "tmux session $session_name already exists."
    fi
done

echo "All experiments started. Use 'tmux ls' to see all sessions."
