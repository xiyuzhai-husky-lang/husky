import json
import matplotlib.pyplot as plt
import numpy as np
import os
import torch
import pdb
from brokenaxes import brokenaxes

DATASETS = [
    "n100000-f10-a5-c5-d3-v0.20-e0.50",
]
exp_dir = "results"

I1R = 400 * (10**4)
I2L = 1550 * (10**4)
I2R = 1650 * (10**4)

RUNS = os.listdir(exp_dir)

plt.rcParams.update({'font.size': 18})

for dataset in DATASETS:
    runs = [run for run in RUNS if dataset in run]

    val_dict = {}
    for run in runs:
        parts = run.split("_")
        model = parts[0]

        run_dir = os.path.join(exp_dir, run)

        config = json.load(open(os.path.join(run_dir, "config.json")))
        if "total_params" in config:
            total_params = config["total_params"]
        else:
            ckpts = [x for x in os.listdir(run_dir) if x.endswith(".pth")]
            if not ckpts:
                print(f"Skipping {run} as no checkpoints found")
                continue
            weights = torch.load(os.path.join(run_dir, ckpts[0]), map_location="cpu")
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
    models = []
    for metric in val_dict:
        for model in val_dict[metric]:
            models.append(model)
    model_order = sorted(list(set(models)))
    color_dict = {model: i for i, model in enumerate(model_order)}

    for metric in val_dict:
        fig = plt.figure(figsize=(6, 6))
        if "acc" in metric:
            bax = brokenaxes(xlims=((0, I1R), (I2L, I2R)), width_ratios=[5, 1],
                             ylims=((0, 0.05), (0.6, 1.01)),
                             hspace=.05, fig=fig)
        else:
            bax = brokenaxes(xlims=((0, I1R), (I2L, I2R)), width_ratios=[5, 1],
                             hspace=.05, fig=fig)
            bax.set_ylim(bottom=0, top=1.5)
        for model in model_order:
            if model not in val_dict[metric]:
                continue

            for param in val_dict[metric][model]:
                val_dict[metric][model][param] = np.mean(val_dict[metric][model][param])
            
            x, y = zip(*sorted(val_dict[metric][model].items()))
            x, y = np.array(x), np.array(y)
            scatter = bax.scatter(x, y, label=model, color=colors[color_dict[model]], s=9**2)

            idx = x <= I1R
            bax.plot(x[idx], y[idx], color=colors[color_dict[model]], linestyle="--", linewidth=3)

        bax.ticklabel_format(style='sci', axis='x', scilimits=(4,4))

        bax.set_xlabel("#Params")
        bax.set_ylabel(metric)
        bax.set_title(dataset)

        bax.legend(loc="lower right" if "acc" in metric else "upper right")

        bax.grid(True)
        os.makedirs(f"figures/{dataset}", exist_ok=True)
        plt.savefig(f"figures/{dataset}/{metric}.pdf")
        plt.close()