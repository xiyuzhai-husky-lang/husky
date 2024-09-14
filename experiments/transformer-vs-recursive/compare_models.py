import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader
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
    "num_epochs": 10,
    "learning_rate": 1e-3,
    "hidden_dim": 64,
    "d_model": 64,
    "num_heads": 4,
    "num_layers": 8,
}

# Load the dataset
dataset = MiniHuskyDataset(100000, 20, 0.10)


def custom_collate(batch):
    inputs, targets = zip(*batch)
    inputs = [torch.as_tensor(x) for x in inputs]  # Convert lists to tensors
    inputs_padded = pad_sequence(inputs, batch_first=True, padding_value=0)
    targets_padded = pad_sequence(targets, batch_first=True, padding_value=0)
    return inputs_padded, targets_padded


dataloader = DataLoader(
    dataset, batch_size=config["batch_size"], shuffle=True, collate_fn=custom_collate
)

# Initialize wandb
wandb.init(project="transformer-vs-rnn", config=config)

# Set device to CUDA if available
device = torch.device("cuda:0" if torch.cuda.is_available() else "cpu")
print(f"Using device: {device}")

# Create models
vocab_size = len(dataset.vocab)
transformer = EncoderOnlyTransformer(
    input_dim=vocab_size,
    output_dim=1,
    num_layers=config["num_layers"],
    num_heads=config["num_heads"],
    d_model=config["d_model"],
    max_seq_len=1000,
).to(device)

rnn = SimpleRNN(input_dim=vocab_size, hidden_dim=config["hidden_dim"], output_dim=1).to(
    device
)

# Loss function and optimizers
criterion = nn.BCEWithLogitsLoss()
transformer_optimizer = optim.Adam(transformer.parameters(), lr=config["learning_rate"])
rnn_optimizer = optim.Adam(rnn.parameters(), lr=config["learning_rate"])

# Train the models
print("Training Transformer...")
train_model(
    transformer,
    dataloader,
    criterion,
    transformer_optimizer,
    config["num_epochs"],
    device=device,  # Add this line
    log_wandb=True,
    model_name="Transformer",
)

print("Training RNN...")
train_model(
    rnn,
    dataloader,
    criterion,
    rnn_optimizer,
    config["num_epochs"],
    device=device,  # Add this line
    log_wandb=True,
    model_name="RNN",
)

wandb.finish()
