import os
import re

import torch


def save_model(rnn, model_name):
    os.makedirs("models", exist_ok=True)
    try:
        os.remove(os.path.join("models", model_name))
    except:
        pass
    torch.save(rnn, os.path.join("models", model_name))


def load_model(path, device="cuda"):
    model = torch.load(path, map_location=torch.device(device))
    return model.to(device)


def read_log(path):
    pattern = r"\D*(\d*)\D*(\d*.\d*)"
    epochs, losses = zip(*[re.findall(pattern, line)[0] for line in open(path, "r")])

    epochs = [int(epoch) for epoch in epochs]
    losses = [float(loss) for loss in losses]

    return epochs, losses