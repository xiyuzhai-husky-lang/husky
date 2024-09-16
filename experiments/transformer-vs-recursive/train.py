import wandb
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader


def train_model(
    model,
    dataloader,
    criterion,
    optimizer,
    num_epochs,
    device,
    output_dims,  # Required parameter for output dimensions
    log_wandb=True,
    model_name=None,
    padding_value=-1,
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
        output_dims (tuple): Dimensions of (AST, symbol, error) outputs along the channel dimension.
        log_wandb (bool): Whether to log metrics to wandb.
        model_name (str): Name of the model for logging purposes.

    Returns:
        None
    """
    ast_dim, symbol_dim, error_dim = output_dims

    model.to(device)
    for epoch in range(num_epochs):
        model.train()
        total_loss = 0.0
        total_ast_acc = 0.0
        total_symbol_acc = 0.0
        total_error_acc = 0.0

        for batch_idx, (inputs, targets) in enumerate(dataloader):
            inputs = inputs.to(device)
            ast_targets, symbol_targets, error_targets = [t.to(device) for t in targets]

            # Zero the parameter gradients
            optimizer.zero_grad()

            # Forward pass
            outputs = model(inputs)  # Shape: B*L*C
            assert (
                outputs.shape[2] >= ast_dim + symbol_dim + error_dim
            ), f"Output dimension {outputs.shape[2]} is less than the sum of ast_dim ({ast_dim}), symbol_dim ({symbol_dim}), and error_dim ({error_dim})"

            # Split the outputs tensor along the third dimension (C)
            ast_outputs = outputs[:, :, :ast_dim]
            symbol_outputs = outputs[:, :, ast_dim : ast_dim + symbol_dim]
            error_outputs = outputs[
                :, :, ast_dim + symbol_dim : ast_dim + symbol_dim + error_dim
            ]

            # Reshape outputs and targets for loss calculation (combine B and L dimensions)
            ast_outputs = ast_outputs.view(-1, ast_dim)
            symbol_outputs = symbol_outputs.view(-1, symbol_dim)
            error_outputs = error_outputs.view(-1, error_dim)

            ast_targets = ast_targets.view(-1)
            symbol_targets = symbol_targets.view(-1)
            error_targets = error_targets.view(-1)

            # Calculate losses
            ast_mask = ast_targets != padding_value
            symbol_mask = symbol_targets != padding_value
            error_mask = error_targets != padding_value

            ast_loss = criterion(ast_outputs[ast_mask], ast_targets[ast_mask])
            symbol_loss = criterion(
                symbol_outputs[symbol_mask], symbol_targets[symbol_mask]
            )
            error_loss = criterion(error_outputs[error_mask], error_targets[error_mask])

            # Combine losses (you can adjust weights if needed)
            combined_loss = ast_loss + symbol_loss + error_loss

            # Backward pass and optimize
            combined_loss.backward()
            optimizer.step()

            total_loss += combined_loss.item()

            # Calculate accuracies
            ast_acc = (
                (
                    (ast_outputs.argmax(dim=1) == ast_targets)
                    & (ast_targets != padding_value)
                )
                .float()
                .mean()
            )
            symbol_acc = (
                (
                    (symbol_outputs.argmax(dim=1) == symbol_targets)
                    & (symbol_targets != padding_value)
                )
                .float()
                .mean()
            )
            error_acc = (
                (
                    (error_outputs.argmax(dim=1) == error_targets)
                    & (error_targets != padding_value)
                )
                .float()
                .mean()
            )

            total_ast_acc += ast_acc.item()
            total_symbol_acc += symbol_acc.item()
            total_error_acc += error_acc.item()

        # Calculate average loss and accuracies for the epoch
        avg_loss = total_loss / len(dataloader)
        avg_ast_acc = total_ast_acc / len(dataloader)
        avg_symbol_acc = total_symbol_acc / len(dataloader)
        avg_error_acc = total_error_acc / len(dataloader)

        if log_wandb:
            wandb.log(
                {
                    f"{model_name}_loss": avg_loss,
                    f"{model_name}_ast_accuracy": avg_ast_acc,
                    f"{model_name}_symbol_accuracy": avg_symbol_acc,
                    f"{model_name}_error_accuracy": avg_error_acc,
                },
                step=epoch,
            )

        print(
            f"Epoch [{epoch + 1}/{num_epochs}], Loss: {avg_loss:.4f}, "
            f"AST Acc: {avg_ast_acc:.4f}, Symbol Acc: {avg_symbol_acc:.4f}, Error Acc: {avg_error_acc:.4f}"
        )
