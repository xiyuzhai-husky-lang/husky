import argparse
import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader, random_split
import wandb
from datasets.mini_husky import MiniHuskyDataset
from models.rnn import SimpleRNN
from train import train_model
from utils import set_seed, custom_collate, linear_warmup_decay, Logger, ordered_search_space

import os
import pdb

HIDDEN_DIM_SPACE = list(range(8, 64 + 1, 8)) + [256]
BATCH_SIZE = 512

parser = argparse.ArgumentParser(description="Train RNN models with different configurations.")
parser.add_argument('--dataset', type=str, default="n100000-f20-d5-v0.20-e0.50", help='Dataset to use')
parser.add_argument('--num_epochs', type=int, default=50, help='Number of epochs to train')
parser.add_argument('--seed', type=int, default=123, help='Random seed for initialization')
parser.add_argument('--server_name', type=str, default="")
parser.add_argument('--gpu_id', type=int, default=0)
parser.add_argument('--try_hidden_dim', type=int, default=None)
args = parser.parse_args()

dataset = MiniHuskyDataset(os.path.join(os.environ["DATA_ROOT"],
                                        "mini-husky/basic",
                                        f"dataset-{args.dataset}.msgpack"))
header = dataset.header

# Split the dataset into train and validation sets
train_size = int(0.8 * len(dataset))  # 80% for training
val_size = len(dataset) - train_size  # 20% for validation

# fix dataset
set_seed(0)
train_dataset, val_dataset = random_split(dataset, [train_size, val_size])

def run(config, train_dataset, val_dataset, header):
    set_seed(config["seed"])

    train_dataloader = DataLoader(
        train_dataset,
        batch_size=config["batch_size"],
        shuffle=True,
        collate_fn=custom_collate,
        num_workers=4,
    )
    val_dataloader = DataLoader(
        val_dataset,
        batch_size=config["batch_size"],
        shuffle=False,
        collate_fn=custom_collate,
        num_workers=4,
    )

    exp_name = f"rnn_hd{config['hidden_dim']}_l{config['num_layers']}_seed{config['seed']}_{args.dataset}"

    logger = Logger(
        exp_root=os.path.join(os.environ["EXP_ROOT"], "transformer_vs_rnn"),
        exp_name=exp_name,
        log_wandb=True,
        config=config
    )

    device = torch.device(f"cuda:{config['gpu_id']}" if torch.cuda.is_available() else "cpu")
    print(f"Using device: {device}")

    # Create models
    model = SimpleRNN(
        input_dim=len(dataset.vocab),
        output_dim=sum(dataset.get_output_dims()),
        bidirectional=True,
        **config
    ).to(device)

    # Loss function and optimizers
    criterion = nn.CrossEntropyLoss(reduction="sum", ignore_index=-1)
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
    model = train_model(
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
        output_dims=dataset.get_output_dims(),
        logger=logger,
    )

    logger.finish()
    torch.save(model.state_dict(), os.path.join(logger.exp_path, "model.pth"))

if args.try_hidden_dim is not None:
    print("Running with hidden_dim:", args.try_hidden_dim)
    search_space = [args.try_hidden_dim]
else:
    search_space = HIDDEN_DIM_SPACE

for hidden_dim in ordered_search_space(search_space):
    min_lr, max_lr = 1e-5, 1e-3
    
    micro_batch_size = BATCH_SIZE

    config = {
        **vars(args),
        "batch_size": BATCH_SIZE,
        "micro_batch_size": micro_batch_size,
        "min_lr": min_lr,
        "max_lr": max_lr,
        "warmup_iters": 990,
        "hidden_dim": hidden_dim,
        "num_layers": 8,
    }
    run(config, train_dataset, val_dataset, header)
