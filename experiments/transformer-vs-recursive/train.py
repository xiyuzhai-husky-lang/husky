import wandb
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader


def train_model(
    model, dataloader, criterion, optimizer, num_epochs, device="cpu", log_wandb=True
):
    """
    Generic training function for models like Transformers, RNNs, etc.

    Args:
        model (torch.nn.Module): The model to train.
        dataloader (torch.utils.data.DataLoader): DataLoader for the training data.
        criterion (torch.nn.Module): Loss function.
        optimizer (torch.optim.Optimizer): Optimizer.
        num_epochs (int): Number of epochs to train for.
        device (str): Device to train on ('cpu' or 'cuda').
        log_wandb (bool): Whether to log metrics to wandb.

    Returns:
        None
    """
    model.to(device)
    for epoch in range(num_epochs):
        model.train()
        total_loss = 0.0

        for batch_idx, (inputs, targets) in enumerate(dataloader):
            inputs, targets = inputs.to(device), targets.to(device)

            # Zero the parameter gradients
            optimizer.zero_grad()

            # Forward pass
            outputs = model(inputs)
            loss = criterion(outputs, targets)

            # Backward pass and optimize
            loss.backward()
            optimizer.step()

            total_loss += loss.item()

        # Calculate average loss for the epoch
        avg_loss = total_loss / len(dataloader)

        if log_wandb:
            wandb.log({"epoch": epoch + 1, "loss": avg_loss})

        print(f"Epoch [{epoch + 1}/{num_epochs}], Loss: {avg_loss:.4f}")
