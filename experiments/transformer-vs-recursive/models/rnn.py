import torch
import torch.nn as nn
import torch.nn.functional as F

class RNNEncoder(nn.Module):
    def __init__(self, vocab_size, hidden_size, num_layers):
        super().__init__()
        self.embedding = nn.Embedding(vocab_size, hidden_size)
        self.rnn = nn.GRU(hidden_size, hidden_size, num_layers, batch_first=True)
        self.fc = nn.Linear(hidden_size, 1)

    def forward(self, x):
        x = self.embedding(x)
        output, _ = self.rnn(x)
        return self.fc(output).squeeze(-1)

class SimpleRNN(nn.Module):
    def __init__(self, input_dim, hidden_dim, output_dim, bidirectional=True):
        super(SimpleRNN, self).__init__()
        self.input_dim = input_dim
        self.rnn = nn.RNN(input_dim, hidden_dim, batch_first=True, bidirectional=bidirectional)
        if bidirectional:
            hidden_dim *= 2
        self.fc = nn.Linear(hidden_dim, output_dim)

    def forward(self, x):
        output, _ = self.rnn(F.one_hot(x, num_classes=self.input_dim).float())
        return self.fc(output)
