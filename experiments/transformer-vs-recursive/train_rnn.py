import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader, random_split
import wandb
from datasets.mini_husky import MiniHuskyDataset
from models.rnn import SimpleRNN
from train import train_model, eval_model
from utils import set_seed, custom_collate

import os
import pdb


# Configurations
config = {
    "seed": 42,
    "batch_size": 512,
    "micro_batch_size": 512,
    "num_epochs": 4,
    "learning_rate": 1e-4,
    "hidden_dim": 4,
}

set_seed(config["seed"])

# Load the dataset
dataset = MiniHuskyDataset(
    n=1000000,
    max_fns=20,
    error_rate=0.50,
    data_dir=os.path.join(os.environ["DATA_ROOT"], "mini-husky/basic")
)

# Add this line near the top of the file, after loading the dataset
output_dims = dataset.get_output_dims()  # Assuming this method exists
output_dim = sum(output_dims)  # Sum up all the values in output_dims

# Split the dataset into train and validation sets
train_size = int(0.8 * len(dataset))  # 80% for training
val_size = len(dataset) - train_size  # 20% for validation
train_dataset, val_dataset = random_split(dataset, [train_size, val_size])

# Create train and validation dataloaders
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

# Initialize wandb
wandb.init(project="transformer-vs-rnn", name=exp_name, config=config)

# Set device to CUDA if available
device = torch.device("cuda:0" if torch.cuda.is_available() else "cpu")
print(f"Using device: {device}")

# Create models
vocab_size = len(dataset.vocab)

rnn = SimpleRNN(
    input_dim=vocab_size,
    hidden_dim=config["hidden_dim"],
    output_dim=output_dim,  # Updated to use output_dims from dataset
).to(device)

# Loss function and optimizers
criterion = nn.CrossEntropyLoss(reduction="sum", ignore_index=-1)
rnn_optimizer = optim.Adam(rnn.parameters(), lr=config["learning_rate"])

# Train the model

print("Training RNN...")
rnn_best_model = train_model(
    model=rnn,
    train_dataloader=train_dataloader,
    val_dataloader=val_dataloader,
    criterion=criterion,
    optimizer=rnn_optimizer,
    num_epochs=config["num_epochs"],
    micro_batch_size=config["micro_batch_size"],
    device=device,  # Add this line
    output_dims=output_dims,  # Use the retrieved output_dims
    log_wandb=True,
)

print("Evaluating RNN...")
eval_model(
    model=rnn_best_model,
    val_dataloader=val_dataloader,
    criterion=criterion,
    device=device,
    output_dims=output_dims,
    micro_batch_size=config["micro_batch_size"] * 4,
)

wandb.finish()
