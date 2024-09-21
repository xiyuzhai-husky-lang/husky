import numpy as np
import random
import torch
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
    ast_kinds, symbol_resolutions, errors = zip(*targets)
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
    return inputs_padded, (ast_kinds_padded, symbol_resolutions_padded, errors_padded)

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