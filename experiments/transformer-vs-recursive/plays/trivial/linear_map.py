#!/usr/bin/env python3

from models import EncoderOnlyTransformer
from dataset import RandomDataset
from train import *
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader


# Customizable Operation
def operation(x):
    # Define your operation here, e.g., element-wise square
    return x**2


# Example usage
if __name__ == "__main__":
    wandb.init(project="play1")
    # Configurations
    config = {
        "batch_size": 32,
        "num_epochs": 10,
        "learning_rate": 1e-3,
    }

    # Sample Dataset and DataLoader (replace with your dataset)
    dataset = RandomDataset(
        num_samples=1000, seq_len=10, input_dim=8, output_dim=8, operation=operation
    )
    dataloader = DataLoader(dataset, batch_size=config["batch_size"], shuffle=True)

    # Sample Model (replace with your model)
    model = EncoderOnlyTransformer(
        input_dim=8,
        output_dim=8,
        num_layers=2,
        num_heads=2,
        d_model=128,
        max_seq_len=10,
    )

    # Loss function and optimizer
    criterion = nn.MSELoss()
    optimizer = optim.Adam(model.parameters(), lr=config["learning_rate"])

    # Train the model
    train_model(
        model,
        dataloader,
        criterion,
        optimizer,
        num_epochs=config["num_epochs"],
        log_wandb=True,
    )
