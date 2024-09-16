import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader, random_split
import wandb
from datasets.mini_husky import MiniHuskyDataset
from models.transformer import EncoderOnlyTransformer
from train import train_model
from torch.nn.utils.rnn import pad_sequence


# Define a simple RNN model
class SimpleRNN(nn.Module):
    def __init__(self, input_dim, hidden_dim, output_dim):
        super(SimpleRNN, self).__init__()
        self.rnn = nn.RNN(input_dim, hidden_dim, batch_first=True)
        self.fc = nn.Linear(hidden_dim, output_dim)

    def forward(self, x):
        _, hidden = self.rnn(x)
        return self.fc(hidden.squeeze(0))


# Configurations
config = {
    "batch_size": 32,
    "num_epochs": 100,
    "learning_rate": 1e-3,
    "hidden_dim": 64,
    "d_model": 256,
    "num_heads": 4,
    "num_layers": 12,
}

# Load the dataset
dataset = MiniHuskyDataset(100000, 20, 0.50)

# Add this line near the top of the file, after loading the dataset
output_dims = dataset.get_output_dims()  # Assuming this method exists
output_dim = sum(output_dims)  # Sum up all the values in output_dims


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
criterion = nn.CrossEntropyLoss()
transformer_optimizer = optim.Adam(transformer.parameters(), lr=config["learning_rate"])
rnn_optimizer = optim.Adam(rnn.parameters(), lr=config["learning_rate"])

# Train the models
print("Training Transformer...")
train_model(
    transformer,
    train_dataloader,
    val_dataloader,
    criterion,
    transformer_optimizer,
    config["num_epochs"],
    device=device,  # Add this line
    log_wandb=True,
    model_name="Transformer",
    output_dims=output_dims,  # Use the retrieved output_dims
)

print("Training RNN...")
train_model(
    rnn,
    train_dataloader,
    val_dataloader,
    criterion,
    rnn_optimizer,
    config["num_epochs"],
    device=device,  # Add this line
    log_wandb=True,
    model_name="RNN",
    output_dims=output_dims,  # Use the retrieved output_dims
)

wandb.finish()
