import torch
import torch.nn as nn


class EncoderOnlyTransformer(nn.Module):
    def __init__(
        self, input_dim, output_dim, num_layers, num_heads, dim_feedforward, max_seq_len
    ):
        super(EncoderOnlyTransformer, self).__init__()
        self.embedding = nn.Linear(input_dim, dim_feedforward)
        self.positional_encoding = nn.Parameter(
            torch.zeros(1, max_seq_len, dim_feedforward)
        )
        encoder_layer = nn.TransformerEncoderLayer(
            d_model=dim_feedforward, nhead=num_heads
        )
        self.transformer_encoder = nn.TransformerEncoder(
            encoder_layer, num_layers=num_layers
        )
        self.fc_out = nn.Linear(dim_feedforward, output_dim)

    def forward(self, x):
        x = self.embedding(x) + self.positional_encoding[:, : x.size(1), :]
        x = self.transformer_encoder(x)
        x = self.fc_out(x)
        return x


eotf = EncoderOnlyTransformer(10, 10, 3, 2, 10, 100)
print(eotf(torch.zeros(1, 10, 10)))
