import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader, random_split
import wandb
from datasets.mini_husky import MiniHuskyDataset
from models.rnn import SimpleRNN
from train import train_model
from utils import set_seed, custom_collate, Logger

import os
import pdb
# Load the dataset
dataset = MiniHuskyDataset(
    n=10000,
    max_fns=100,
    use_var_rate=0.2,
    error_rate=0.2,
    data_dir=os.path.join(os.environ["DATA_ROOT"], "mini-husky/basic")
)
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

    exp_name = f"rnn_{config['hidden_dim']}_seed{config['seed']}_bs{config['batch_size']}"

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
        hidden_dim=config["hidden_dim"],
        output_dim=output_dim,  # Updated to use output_dims from dataset
    ).to(device)

    # Loss function and optimizers
    criterion = nn.CrossEntropyLoss(reduction="sum", ignore_index=-1)
    optimizer = optim.Adam(model.parameters(), lr=config["learning_rate"])

    # Train the model
    print("Training RNN...")
    best_model = train_model(
        model=model,
        header=header,
        train_dataloader=train_dataloader,
        val_dataloader=val_dataloader,
        criterion=criterion,
        optimizer=optimizer,
        num_epochs=config["num_epochs"],
        micro_batch_size=config["micro_batch_size"],
        device=device,
        output_dims=output_dims,
        logger=logger,
    )

    logger.finish()
    torch.save(best_model.state_dict(), os.path.join(logger.exp_path, "best_model.pth"))

for hidden_dim in [64, 32, 16, 8, 4]:
    config = {
        "seed": 42,
        "batch_size": 512,
        "micro_batch_size": 128,  # Assuming a change is needed here
        "num_epochs": 100,
        "learning_rate": 2e-4,
        "hidden_dim": hidden_dim,
    }

    run(config)
