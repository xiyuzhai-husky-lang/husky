import json
import matplotlib.pyplot as plt
import numpy as np
import os
import torch
import pdb

# DATASET = "n100000-f10-d3-v0.20-e0.50"
DATASET = "n100000-f20-d5-v0.20-e0.50"
exp_dir = "results"

runs = os.listdir(exp_dir)
runs = [run for run in runs if DATASET in run]

val_dict = {}
for run in runs:
    parts = run.split("_")
    model = parts[0]

    run_dir = os.path.join(exp_dir, run)
    ckpts = [x for x in os.listdir(run_dir) if x.endswith(".pth")]
    if not ckpts:
        print(f"Skipping {run} as no checkpoints found")
        continue
    weights = torch.load(os.path.join(run_dir, ckpts[0]), map_location="cpu")

    # get total param count
    total_params = 0
    for param in weights:
        total_params += weights[param].numel()

    # read from jsonl file for log
    log = []
    with open(os.path.join(run_dir, "log.jsonl"), "r") as f:
        for line in f:
            if not line.strip():
                continue
            log.append(json.loads(line))
    
    local_dict = {}
    for l in log:
        if "val/loss" in l:
            for k, v in l.items():
                if "val/" in k:
                    nk = k[4:]
                    if nk not in local_dict:
                        local_dict[nk] = []
                    local_dict[nk].append(v)
            
    for k, v in local_dict.items():
        if k not in val_dict:
            val_dict[k] = {}
        if model not in val_dict[k]:
            val_dict[k][model] = {}
        if total_params not in val_dict[k][model]:
            val_dict[k][model][total_params] = []
        v = sorted(v)
        if "acc" in k:
            metric = np.mean(v[-5:])
        else:
            metric = np.mean(v[:5])
        val_dict[k][model][total_params].append(metric)

colors = plt.cm.tab10(np.linspace(0, 1, 10))
color_dict = {}

for metric in val_dict:
    # new plot
    plt.figure()
    for model in val_dict[metric]:
        if model not in color_dict:
            color_dict[model] = len(color_dict)

        for param in val_dict[metric][model]:
            val_dict[metric][model][param] = np.mean(val_dict[metric][model][param])
        
        x, y = zip(*sorted(val_dict[metric][model].items()))
        plt.scatter(x, y, label=model, color=colors[color_dict[model]])
        plt.plot(x, y, color=colors[color_dict[model]], linestyle="--")

    plt.ylim(bottom=0)
    if "acc" in metric:
        plt.ylim(top=1.1)

    # plt.xscale("log")
    plt.xlabel("#Params")
    plt.ylabel(metric)
    plt.title(DATASET)
    plt.legend()
    plt.grid(True)
    plt.tight_layout()
    os.makedirs("figures", exist_ok=True)
    plt.savefig(f"figures/{metric}.pdf")

