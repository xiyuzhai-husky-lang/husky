import argparse
import numpy as np
import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader, random_split
import wandb
from datasets.mini_husky import MiniHuskyDataset
from models.rnn import SimpleRNN
from train import train_model
from utils import set_seed, custom_collate, linear_warmup_decay, Logger

import os
import pdb

HIDDEN_DIM_SPACE = [1, 2, 4, 8, 16] + list(range(32, 512 + 1, 32))

DATASET = "n100000-f10-d3-v0.20-e0.50"
dataset = MiniHuskyDataset(os.path.join(os.environ["DATA_ROOT"],
                                        "mini-husky/basic",
                                        f"dataset-{DATASET}.msgpack"))
header = dataset.header
vocab_size = len(dataset.vocab)
output_dims = dataset.get_output_dims()
output_dim = sum(output_dims)

# Split the dataset into train and validation sets
train_size = int(0.8 * len(dataset))  # 80% for training
val_size = len(dataset) - train_size  # 20% for validation
train_dataset, val_dataset = random_split(dataset, [train_size, val_size])

def run(config):
    set_seed(config["seed"])

    train_dataloader = DataLoader(
        train_dataset,
        batch_size=config["batch_size"],
        shuffle=True,
        collate_fn=custom_collate,
    )
    val_dataloader = DataLoader(
        val_dataset,
        batch_size=config["batch_size"],
        shuffle=False,
        collate_fn=custom_collate,
    )

    exp_name = f"rnn_hd{config['hidden_dim']}_l{config['num_layers']}_seed{config['seed']}_{DATASET}"

    logger = Logger(
        exp_root=os.path.join(os.environ["EXP_ROOT"], "transformer_vs_rnn"),
        exp_name=exp_name,
        log_wandb=True,
        config=config
    )

    # Set device to CUDA if available
    device = torch.device("cuda:0" if torch.cuda.is_available() else "cpu")
    print(f"Using device: {device}")

    # Create models
    model = SimpleRNN(
        input_dim=vocab_size,
        output_dim=output_dim,
        bidirectional=True,
        **config
    ).to(device)

    # Loss function and optimizers
    criterion = nn.CrossEntropyLoss(reduction="sum", ignore_index=0)
    optimizer = optim.Adam(model.parameters(), lr=1)
    scheduler = torch.optim.lr_scheduler.LambdaLR(
        optimizer,
        lr_lambda=linear_warmup_decay(
            total_iters=config["num_epochs"] * len(train_dataloader),
            **config
        )
    )

    # Train the model
    print("Training RNN...")
    best_model = train_model(
        model=model,
        header=header,
        train_dataloader=train_dataloader,
        val_dataloader=val_dataloader,
        criterion=criterion,
        optimizer=optimizer,
        scheduler=scheduler,
        num_epochs=config["num_epochs"],
        micro_batch_size=config["micro_batch_size"],
        device=device,
        output_dims=output_dims,
        logger=logger,
    )

    logger.finish()
    torch.save(best_model.state_dict(), os.path.join(logger.exp_path, "best_model.pth"))

parser = argparse.ArgumentParser(description="Train RNN models with different configurations.")
parser.add_argument('--seed', type=int, default=42, help='Random seed for initialization')
args = parser.parse_args()
seed = args.seed

# for seed in [42, 142857, 2225393, 20000308, 2018011309]:
for hidden_dim in reversed(HIDDEN_DIM_SPACE):
    if hidden_dim <= 160:
        min_lr = 1e-5
        max_lr = 1e-3
    else:
        min_lr = 1e-6
        max_lr = 1e-4

    config = {
        "seed": seed,
        "batch_size": 512,
        "micro_batch_size": 512,
        "num_epochs": 100,
        "min_lr": min_lr,
        "max_lr": max_lr,
        "warmup_iters": 990,
        "hidden_dim": hidden_dim,
        "num_layers": 8,
    }

    run(config)
