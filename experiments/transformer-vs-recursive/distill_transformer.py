import argparse
import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader, random_split
import json
from datasets.mini_husky import MiniHuskyDataset
from models.transformer import EncoderOnlyTransformer, CustomBERTModel
from train import train_model
from utils import set_seed, custom_collate, linear_warmup_decay, Logger, ordered_search_space

import os
import pdb

# Define the hidden dimension space for the student model
HIDDEN_DIM_SPACE = [4]
BATCH_SIZE = 512

# Argument parsing
parser = argparse.ArgumentParser(description="Distill Transformer models with different configurations.")
parser.add_argument('--dataset', type=str, default="100000-f10-d3-v0.20-e0.50", help='Dataset to use')
parser.add_argument('--num_epochs', type=int, default=10, help='Number of epochs to train')
parser.add_argument('--seed', type=int, default=42, help='Random seed for initialization')
parser.add_argument('--server_name', type=str, default="")
parser.add_argument('--gpu_id', type=int, default=0)
parser.add_argument('--teacher_path', type=str, required=True, help='Path to the teacher model')
args = parser.parse_args()

device = torch.device(f"cuda:{args.gpu_id}" if torch.cuda.is_available() else "cpu")

# Load the dataset
dataset = MiniHuskyDataset(os.path.join(os.environ["DATA_ROOT"], "mini-husky/basic", f"dataset-{args.dataset}.msgpack"))
header = dataset.header
max_seq_len = ((dataset.get_max_len() - 1) // 512 + 1) * 512

# Split the dataset
train_size = int(0.8 * len(dataset))  # 80% for training
val_size = len(dataset) - train_size  # Remaining for validation
set_seed(0)
train_dataset, val_dataset = random_split(dataset, [train_size, val_size])

# Load the teacher model
teacher_path = os.path.join(os.environ["EXP_ROOT"], "transformer_vs_rnn/transformer_teacher", args.teacher_path)
teacher_config = json.load(open(os.path.join(teacher_path, "config.json")))
teacher_model = CustomBERTModel(output_dim=sum(dataset.get_output_dims()), 
                                vocab_size=len(dataset.vocab),
                                max_seq_len=max_seq_len,
                                **teacher_config).to(device)

weight_files = [x for x in os.listdir(teacher_path) if x.endswith(".pth")]
teacher_model.load_state_dict(torch.load(os.path.join(teacher_path, weight_files[0])))
teacher_model.eval()

def run(config, train_dataset, val_dataset, header):
    set_seed(config["seed"])

    # Data loaders
    train_dataloader = DataLoader(train_dataset, batch_size=config["batch_size"], shuffle=True, collate_fn=custom_collate)
    val_dataloader = DataLoader(val_dataset, batch_size=config["batch_size"], shuffle=False, collate_fn=custom_collate)

    # Experiment name
    exp_name = f"distilled_transformer_d{config['d_model']}_h{config['num_heads']}_l{config['num_layers']}_seed{config['seed']}_{args.dataset}"

    # Logger setup
    logger = Logger(exp_root=os.path.join(os.environ["EXP_ROOT"], "transformer_distillation"),
                    exp_name=exp_name, log_wandb=True, config=config)

    # Initialize the student model
    model = CustomBERTModel(output_dim=sum(dataset.get_output_dims()), **config).to(device)

    criterion = nn.CrossEntropyLoss(reduction="sum", ignore_index=0)
    optimizer = optim.Adam(model.parameters(), lr=1)
    scheduler = torch.optim.lr_scheduler.LambdaLR(optimizer, lr_lambda=linear_warmup_decay(total_iters=config["num_epochs"] * len(train_dataloader), **config))

    print("Distilling Transformer...")
    model = train_model(model=model, header=header, train_dataloader=train_dataloader, val_dataloader=val_dataloader, criterion=criterion, optimizer=optimizer, scheduler=scheduler, num_epochs=config["num_epochs"], micro_batch_size=config["micro_batch_size"], device=device, output_dims=dataset.get_output_dims(), logger=logger, teacher_model=teacher_model)

    torch.save(model.state_dict(), os.path.join(logger.exp_path, "student_model.pth"))
    logger.finish()

for hidden_dim in ordered_search_space(HIDDEN_DIM_SPACE):
    micro_batch_size = 512

    config = {
        "batch_size": BATCH_SIZE,
        "micro_batch_size": micro_batch_size,
        "min_lr": 1e-6,
        "max_lr": 1e-4,
        "warmup_iters": 99,
        "vocab_size": len(dataset.vocab),
        "output_dims": dataset.get_output_dims(),
        "d_model": hidden_dim,
        "num_heads": min(4, hidden_dim),
        "num_layers": 8,
        "max_seq_len": max_seq_len,
        **vars(args)
    }

    # Run the distillation process
    run(config, train_dataset, val_dataset, header)
