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
    PADDING_VALUE = 0

    inputs, targets = zip(*batch)
    inputs = [torch.as_tensor(x) for x in inputs]  # Convert lists to tensors
    inputs_padded = pad_sequence(inputs, batch_first=True, padding_value=0)

    # Unpack the targets tuple
    fields = list(zip(*targets))
    for i in range(len(fields)):
        fields[i] = pad_sequence(
            [torch.as_tensor(t) for t in fields[i]],
            batch_first=True,
            padding_value=PADDING_VALUE,
        )

    return inputs_padded, tuple(fields)

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

        self.log_wandb = log_wandb
        if log_wandb:
            wandb.init(project="transformer-vs-rnn", name=f"{exp_name}_{rnd_suf}", config=config)
        
        self.log_buffer = []
        self.buffer_limit = 100
    
    def log(self, data):
        self.log_buffer.append(data)
        if len(self.log_buffer) >= self.buffer_limit:
            self.dump_log()
    
    def dump_log(self):
        with open(self.file_path, "a") as f:
            data_to_write = "\n".join(json.dumps(data) for data in self.log_buffer)
            f.write(data_to_write + "\n")
            for data in self.log_buffer:
                wandb.log(data)
        self.log_buffer = []
    
    def finish(self):
        self.dump_log()
        if self.log_wandb:
            wandb.finish()
        print(f"Logs saved to {self.file_path}")

def ordered_search_space(x):
    if len(x) == 1:
        return x
    y = sorted(x)
    return [y[0], y[-1]] + y[1: -1]