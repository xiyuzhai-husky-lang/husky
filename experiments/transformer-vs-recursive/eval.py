import os
import torch

from datasets.mini_husky import MiniHuskyDataset
from models.transformer import EncoderOnlyTransformer

exp_name = "transformer_64_32_4_2_seed42_bs512_nKYaXRvj"
model_path = os.path.join(os.environ["EXP_ROOT"], "transformer_vs_rnn", exp_name, "best_model.pth")

dataset = MiniHuskyDataset(
    n=100000,
    max_fns=10,
    min_dist=3,
    use_var_rate=0.2,
    error_rate=0.5,
    data_dir=os.path.join(os.environ["DATA_ROOT"], "mini-husky/basic")
)
output_dims = dataset.get_output_dims()

if "transformer" in exp_name:
    parts = exp_name.split("_")
    hidden_dim = int(parts[1])
    d_model = int(parts[2])
    num_heads = int(parts[3])
    num_layers = int(parts[4])

    model = EncoderOnlyTransformer(
        vocab_size=len(dataset.vocab),
        output_dim=sum(output_dims),
        num_layers=num_layers,
        num_heads=num_heads,
        d_model=d_model,
        max_seq_len=4096,
    )
else:
    raise ValueError("Unknown model type")

model.load_state_dict(torch.load(model_path))

inputs, targets = dataset[0]
words = dataset.data[0][0]
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
    if t == 0:
        c = "  "
    else:
        c = "✅" if o == t else "❌"
    print(f"[{c}]", words[i], o, t)
