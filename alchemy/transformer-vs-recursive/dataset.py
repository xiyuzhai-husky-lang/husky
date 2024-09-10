import torch
from torch.utils.data import DataLoader, Dataset


# Dataset Generation
class RandomDataset(Dataset):
    def __init__(self, num_samples, seq_len, input_dim, output_dim, operation):
        self.num_samples = num_samples
        self.seq_len = seq_len
        self.input_dim = input_dim
        self.output_dim = output_dim
        self.operation = operation

    def __len__(self):
        return self.num_samples

    def __getitem__(self, idx):
        x = torch.randn(self.seq_len, self.input_dim)
        y = self.operation(x)
        return x, y
