import os
import torch

from datasets.mini_husky import MiniHuskyDataset
from models.rnn import SimpleRNN
from models.transformer import CustomBERTModel

import pdb

def load_model(exp_name, model_path):
    if "transformer" in exp_name:
        parts = exp_name.split("_")
        d_model = int(parts[1][1:])
        num_heads = int(parts[2][1:])
        num_layers = int(parts[3][1:])

        model = CustomBERTModel(
            vocab_size=vocab_size,
            d_model=d_model,
            output_dim=output_dim,
            num_heads=num_heads,
            num_layers=num_layers,
            max_seq_len=max_seq_len,
        )
    elif "rnn" in exp_name:
        parts = exp_name.split("_")
        hidden_dim = int(parts[1][2:])
        num_layers = int(parts[2][1:])

        model = SimpleRNN(
            input_dim=vocab_size,
            hidden_dim=hidden_dim,
            output_dim=output_dim,
            num_layers=num_layers,
            bidirectional=True,
        )
    else:
        raise ValueError("Unknown model type")

    model.load_state_dict(torch.load(model_path))

    return model

def eval(model, dataset, i):
    inputs, targets = dataset[i]
    words = dataset.data[i][0]
    outputs = model(inputs.unsqueeze(0))

    header = dataset.header
    dim = -1
    for i in range(len(header)):
        if header[i] == "expected_type":
            dim = i
            break
    outputs_by_field = torch.split(outputs, output_dims, dim=2)

    for i in range(len(words)):
        o = outputs_by_field[dim][0, i, :].argmax().item()
        t = targets[dim][i].item()
        # if t == 0:
        #     c = "  "
        # else:
        #     c = "✅" if o == t else "❌"
        # print(f"[{c}]", words[i], o, t)

        if t != 0 and o != t:
            dec_pos = 0
            for j in range(0, i - 2):
                if words[j] == words[i - 2]:
                    dec_pos = j
                    break

            st = ""
            for j in range(i - 2, i + 3):
                if j == i:
                    st += f"[{words[j]}] "
                else:
                    st += f"{words[j]} "
            print(f"{st}: {o} != {t} dist={i - dec_pos}")

# exp_name = "transformer_d128_h4_l8_seed142857_n100000-f10-d3-v0.20-e0.50_DwXb8rVq"
exp_name = "rnn_hd4_l8_seed142857_n100000-f100-d20-v0.20-e0.50_DwXb8rVq"
model_path = os.path.join(os.environ["EXP_ROOT"], "transformer_vs_rnn", exp_name, "best_model.pth")

DATASET = exp_name.split("_")[-2]
dataset = MiniHuskyDataset(os.path.join(os.environ["DATA_ROOT"],
                                        "mini-husky/basic",
                                        f"dataset-{DATASET}.msgpack"))
max_seq_len = ((dataset.get_max_len() - 1) // 512 + 1) * 512
header = dataset.header
vocab_size = len(dataset.vocab)
output_dims = dataset.get_output_dims()
output_dim = sum(output_dims)

model = load_model(exp_name, model_path)

pdb.set_trace()