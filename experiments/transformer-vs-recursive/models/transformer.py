import torch
import torch.nn as nn
import math


class PositionEncoding(nn.Module):
    def __init__(self, d_model, max_seq_len):
        super(PositionEncoding, self).__init__()
        position_encoding = self.create_sinusoidal_encoding(max_seq_len, d_model)
        self.register_buffer("position_encoding", position_encoding)

    def create_sinusoidal_encoding(self, max_seq_len, d_model):
        position = torch.arange(max_seq_len).unsqueeze(1)
        div_term = torch.exp(
            torch.arange(0, d_model, 2) * (-math.log(10000.0) / d_model)
        )
        pe = torch.zeros(1, max_seq_len, d_model)
        pe[0, :, 0::2] = torch.sin(position * div_term)
        pe[0, :, 1::2] = torch.cos(position * div_term)
        return pe

    def forward(self, x):
        return x + self.position_encoding[:, : x.size(1), :]  # pyright: ignore


class EncoderOnlyTransformer(nn.Module):
    def __init__(
        self, vocab_size, output_dim, num_layers, num_heads, d_model, max_seq_len
    ):
        super(EncoderOnlyTransformer, self).__init__()
        self.vocab_size = vocab_size
        self.embedding = nn.Embedding(vocab_size, d_model)
        self.position_encoding = PositionEncoding(d_model, max_seq_len)
        encoder_layer = nn.TransformerEncoderLayer(d_model=d_model, nhead=num_heads)
        self.transformer_encoder = nn.TransformerEncoder(
            encoder_layer, num_layers=num_layers
        )
        self.fc_out = nn.Linear(d_model, output_dim)

    def forward(self, x):
        # Check if input indices are within the valid range
        if torch.any(x >= self.vocab_size) or torch.any(x < 0):
            invalid_indices = torch.where((x >= self.vocab_size) | (x < 0))
            raise ValueError(
                f"Input contains indices outside the valid range [0, {self.vocab_size - 1}]. "
                f"Invalid indices: {invalid_indices}. "
                f"Values at these indices: {x[invalid_indices]}"
            )

        # Convert word indices to embeddings
        x = self.embedding(x)
        x = self.position_encoding(x)
        x = self.transformer_encoder(x)
        x = self.fc_out(x)
        return x


device = "cuda:0"
eotf = EncoderOnlyTransformer(10, 10, 3, 2, 10, 100).to(device)
