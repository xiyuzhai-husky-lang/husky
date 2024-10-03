#!/bin/bash

dataset="n100000-f40-d10-v0.20-e0.50"
num_epochs=20

declare -A gpu_exp_seed_map=(
    [0]="transformer:42"
    [1]="transformer:142857"
    [2]="transformer:2225393"
    [3]="transformer:20000308"
    [4]="transformer:2018011309"
)
# declare -A gpu_exp_seed_map=(
#     [5]="rnn:42"
#     [6]="rnn:142857"
#     [6]="rnn:2225393"
#     [7]="rnn:20000308"
#     [0]="rnn:2018011309"
# )
server_name="fd1"

# Loop through the associative array
for gpu in "${!gpu_exp_seed_map[@]}"; do
    # Extract experiment type and seed using IFS (Internal Field Separator)
    IFS=':' read -r exp seed <<< "${gpu_exp_seed_map[$gpu]}"
    session_name="${exp}_gpu${gpu}_seed${seed}"
    
    # Check if the tmux session already exists and create it if it doesn't
    tmux has-session -t "$session_name" 2>/dev/null
    if [ $? != 0 ]; then
        # Use the experiment type in the command to run the appropriate script
        tmux new-session -d -s "$session_name" "python train_$exp.py --dataset=$dataset --num_epochs=$num_epochs --seed=$seed --server_name=$server_name --gpu_id=$gpu --try_hidden_dim=208"
        echo "Experiment started in tmux session: $session_name"
    else
        echo "tmux session $session_name already exists."
    fi
done

echo "All experiments started. Use 'tmux ls' to see all sessions."
