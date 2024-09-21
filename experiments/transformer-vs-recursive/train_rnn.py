import numpy as np
import random
import torch
import torch.nn as nn
import torch.nn.functional as F
import torch.optim as optim
from torch.utils.data import DataLoader, random_split
import wandb
from datasets.mini_husky import MiniHuskyDataset
from models.rnn import RNNEncoder
from models.transformer import EncoderOnlyTransformer
from train import train_model, eval_model
from torch.nn.utils.rnn import pad_sequence
from utils import set_seed, custom_collate

import pdb

# Define a simple RNN model
class SimpleRNN(nn.Module):
    def __init__(self, input_dim, hidden_dim, output_dim):
        super(SimpleRNN, self).__init__()
        self.input_dim = input_dim
        self.rnn = nn.RNN(input_dim, hidden_dim, batch_first=True)
        self.fc = nn.Linear(hidden_dim, output_dim)

    def forward(self, x):
        output, _ = self.rnn(F.one_hot(x, num_classes=self.input_dim).float())
        return self.fc(output)

# Configurations
config = {
    "seed": 42,
    "batch_size": 1024,
    "micro_batch_size": 64,
    "rnn_micro_batch_size": 1024,
    "num_epochs": 100,
    "learning_rate": 1e-4,
    "hidden_dim": 64,
    "d_model": 256,
    "num_heads": 4,
    "num_layers": 12,
}

set_seed(config["seed"])

# Load the dataset
dataset = MiniHuskyDataset(100000, 20, 0.50)

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

# Initialize wandb
wandb.init(project="transformer-vs-rnn", config=config)

# Set device to CUDA if available
device = torch.device("cuda:0" if torch.cuda.is_available() else "cpu")
# device = "cpu"
print(f"Using device: {device}")

# Create models
vocab_size = len(dataset.vocab)
transformer = EncoderOnlyTransformer(
    vocab_size=vocab_size,
    output_dim=output_dim,  # Updated to use output_dims from dataset
    num_layers=config["num_layers"],
    num_heads=config["num_heads"],
    d_model=config["d_model"],
    max_seq_len=1000,
).to(device)

rnn = SimpleRNN(
    input_dim=vocab_size,
    hidden_dim=config["hidden_dim"],
    output_dim=output_dim,  # Updated to use output_dims from dataset
).to(device)

# Loss function and optimizers
criterion = nn.CrossEntropyLoss(reduction="sum", ignore_index=-1)
transformer_optimizer = optim.Adam(transformer.parameters(), lr=config["learning_rate"])
rnn_optimizer = optim.Adam(rnn.parameters(), lr=config["learning_rate"])

# Train the models
print("Training Transformer...")
transformer_best_model = train_model(
    transformer,
    train_dataloader,
    val_dataloader,
    criterion,
    transformer_optimizer,
    device=device,  # Add this line
    log_wandb=True,
    model_name="Transformer",
    output_dims=output_dims,  # Use the retrieved output_dims
    micro_batch_size=config["micro_batch_size"],
    num_epochs=config["num_epochs"],
)

print("Training RNN...")
rnn_best_model = train_model(
    rnn,
    train_dataloader,
    val_dataloader,
    criterion,
    rnn_optimizer,
    device=device,  # Add this line
    log_wandb=True,
    model_name="RNN",
    output_dims=output_dims,  # Use the retrieved output_dims
    micro_batch_size=config["rnn_micro_batch_size"],
    num_epochs=config["num_epochs"],
)

print("Evaluating Transformer...")
eval_model(
    model=transformer_best_model,
    val_dataloader=val_dataloader,
    criterion=criterion,
    device=device,
    output_dims=output_dims,
    micro_batch_size=config["micro_batch_size"] * 4,
)

print("Evaluating RNN...")
eval_model(
    model=rnn_best_model,
    val_dataloader=val_dataloader,
    criterion=criterion,
    device=device,
    output_dims=output_dims,
    micro_batch_size=config["rnn_micro_batch_size"] * 4,
)

wandb.finish()
