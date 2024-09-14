import torch
import torch.nn as nn
import math


class EncoderOnlyTransformer(nn.Module):
    def __init__(
        self, input_dim, output_dim, num_layers, num_heads, d_model, max_seq_len
    ):
        super(EncoderOnlyTransformer, self).__init__()
        self.embedding = nn.Embedding(input_dim, d_model)
        self.positional_encoding = self.create_sinusoidal_encoding(max_seq_len, d_model)
        encoder_layer = nn.TransformerEncoderLayer(d_model=d_model, nhead=num_heads)
        self.transformer_encoder = nn.TransformerEncoder(
            encoder_layer, num_layers=num_layers
        )
        self.fc_out = nn.Linear(d_model, output_dim)

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
        x = self.embedding(x).to(dtype=torch.float32)
        if x.size(1) > self.positional_encoding.size(1):
            raise ValueError(
                f"Input sequence length {x.size(1)} exceeds maximum context length {self.positional_encoding.size(1)}"
            )
        x = x + self.positional_encoding[:, : x.size(1), :].to(x.device)
        x = self.transformer_encoder(x)
        x = self.fc_out(x)
        return x


device = "cuda:0"
eotf = EncoderOnlyTransformer(10, 10, 3, 2, 10, 100).to(device)
