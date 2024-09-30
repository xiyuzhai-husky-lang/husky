import argparse
import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader, random_split
import wandb
from datasets.mini_husky import MiniHuskyDataset
from models.transformer import EncoderOnlyTransformer, CustomBERTModel
from train import train_model
from utils import set_seed, custom_collate, linear_warmup_decay, Logger, ordered_search_space

import os
import pdb

# Define the hidden dimension space
HIDDEN_DIM_SPACE = [4, 8, 16] + list(range(32, 64 + 1, 16))
BATCH_SIZE = 512

# Argument parsing
parser = argparse.ArgumentParser(description="Train Transformer models with different configurations.")
parser.add_argument('--dataset', type=str, default="100000-f10-d3-v0.20-e0.50", help='Dataset to use')
parser.add_argument('--num_epochs', type=int, default=10, help='Number of epochs to train')
parser.add_argument('--seed', type=int, default=42, help='Random seed for initialization')
parser.add_argument('--server_name', type=str, default="")
parser.add_argument('--gpu_id', type=int, default=0)
parser.add_argument('--try_hidden_dim', type=int, default=None)
args = parser.parse_args()

# Load the dataset
dataset = MiniHuskyDataset(os.path.join(os.environ["DATA_ROOT"],
                                        "mini-husky/basic",
                                        f"dataset-{args.dataset}.msgpack"))
header = dataset.header
max_seq_len = ((dataset.get_max_len() - 1) // 512 + 1) * 512

# Split the dataset into training and validation sets
train_size = int(0.8 * len(dataset))  # 80% for training
val_size = len(dataset) - train_size  # Remaining for validation

# Fix dataset
set_seed(0)
train_dataset, val_dataset = random_split(dataset, [train_size, val_size])

def run(config, train_dataset, val_dataset, header):
    set_seed(config["seed"])

    # Data loaders
    train_dataloader = DataLoader(train_dataset, batch_size=config["batch_size"], shuffle=True, collate_fn=custom_collate)
    val_dataloader = DataLoader(val_dataset, batch_size=config["batch_size"], shuffle=False, collate_fn=custom_collate)

    # Experiment name
    exp_name = f"transformer_d{config['d_model']}_h{config['num_heads']}_l{config['num_layers']}_seed{config['seed']}_{args.dataset}"

    # Logger setup
    logger = Logger(exp_root=os.path.join(os.environ["EXP_ROOT"], "transformer_vs_rnn"),
                    exp_name=exp_name, log_wandb=True, config=config)

    # Device setup
    device = torch.device(f"cuda:{config['gpu_id']}" if torch.cuda.is_available() else "cpu")
    print(f"Using device: {device}")

    # Model creation
    model = CustomBERTModel(output_dim=sum(dataset.get_output_dims()), **config).to(device)

    # Loss function and optimizers
    criterion = nn.CrossEntropyLoss(reduction="sum", ignore_index=0)
    optimizer = optim.Adam(model.parameters(), lr=1)
    scheduler = torch.optim.lr_scheduler.LambdaLR(optimizer, lr_lambda=linear_warmup_decay(total_iters=config["num_epochs"] * len(train_dataloader), **config))

    # Training
    print("Training Transformer...")
    model = train_model(model=model, header=header, train_dataloader=train_dataloader, val_dataloader=val_dataloader, criterion=criterion, optimizer=optimizer, scheduler=scheduler, num_epochs=config["num_epochs"], micro_batch_size=config["micro_batch_size"], device=device, output_dims=dataset.get_output_dims(), logger=logger)

    # Finish logging and save the model
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
        "batch_size": BATCH_SIZE,
        "micro_batch_size": micro_batch_size,
        "min_lr": min_lr,
        "max_lr": max_lr,
        "warmup_iters": 990,
        "vocab_size": len(dataset.vocab),
        "output_dims": dataset.get_output_dims(),
        "d_model": hidden_dim,
        "num_heads": min(4, hidden_dim),
        "num_layers": 8,
        "max_seq_len": max_seq_len,
        **vars(args)
    }

    # Run the training
    run(config, train_dataset, val_dataset, header)
