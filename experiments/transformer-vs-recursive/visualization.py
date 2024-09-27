import json
import matplotlib.pyplot as plt
import numpy as np
import os

DATASET = "n100000-f10-d3-v0.20-e0.50"
MODEL = "rnn"
exp_dir = os.path.join(os.environ["EXP_ROOT"], "transformer_vs_rnn")

runs = os.listdir(exp_dir)
runs = [run for run in runs if MODEL in run and DATASET in run]