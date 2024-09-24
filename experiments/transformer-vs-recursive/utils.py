import json
import numpy as np
import random
import string
import torch
import wandb
import os
from torch.nn.utils.rnn import pad_sequence

def set_seed(seed):
    torch.manual_seed(seed)
    torch.cuda.manual_seed_all(seed)
    torch.cuda.manual_seed(seed)
    torch.backends.cudnn.deterministic = True
    torch.backends.cudnn.benchmark = False
    np.random.seed(seed)
    random.seed(seed)

def custom_collate(batch):
    inputs, targets = zip(*batch)
    inputs = [torch.as_tensor(x) for x in inputs]  # Convert lists to tensors
    inputs_padded = pad_sequence(inputs, batch_first=True, padding_value=0)

    # Unpack the targets tuple
    ast_kinds, symbol_resolutions, errors, expected_types = zip(*targets)
    ast_kinds_padded = pad_sequence(
        [torch.as_tensor(t) for t in ast_kinds], batch_first=True, padding_value=-1
    )
    symbol_resolutions_padded = pad_sequence(
        [torch.as_tensor(t) for t in symbol_resolutions],
        batch_first=True,
        padding_value=-1,
    )
    errors_padded = pad_sequence(
        [torch.as_tensor(t) for t in errors], batch_first=True, padding_value=-1
    )
    expected_types_padded = pad_sequence(
        [torch.as_tensor(t) for t in expected_types],
        batch_first=True,
        padding_value=-1,
    )
    return inputs_padded, (ast_kinds_padded, symbol_resolutions_padded, errors_padded, expected_types_padded)

def linear_warmup_decay(total_iters, warmup_iters, min_lr, max_lr, **kwargs):
    """
    Creates a function that calculates the learning rate for each iteration based on the linear schedule with warmup.
    """
    def lr_lambda(current_iter):
        if current_iter < warmup_iters:
            # Linear warmup
            return min_lr + (max_lr - min_lr) * (current_iter / warmup_iters)
        else:
            # Linear decay
            return max_lr - (max_lr - min_lr) * ((current_iter - warmup_iters) / (total_iters - warmup_iters))
    return lr_lambda

class Logger:
    def __init__(self, exp_root, exp_name, log_wandb, config):
        self.log_wandb = log_wandb
        if log_wandb:
            wandb.init(project="transformer-vs-rnn", name=exp_name, config=config)
        
        while True:
            rnd_suf = "".join(random.choices(string.ascii_letters + string.digits, k=8))
            self.exp_path = os.path.join(exp_root, f"{exp_name}_{rnd_suf}")

            if not os.path.exists(self.exp_path):
                break
        self.file_path = os.path.join(self.exp_path, "log.jsonl")
        os.makedirs(self.exp_path, exist_ok=True)
        with open(os.path.join(self.exp_path, "config.json"), "w") as f:
            json.dump(config, f)
        print(f"Logs will be saved to {self.file_path}")
        
    def log(self, data):
        if self.log_wandb:
            wandb.log(data)
        with open(self.file_path, "a") as f:
            f.write(json.dumps(data) + "\n")
    
    def finish(self):
        if self.log_wandb:
            wandb.finish()
        print(f"Logs saved to {self.file_path}")