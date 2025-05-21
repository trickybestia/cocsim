import torch
import torch.nn as nn
import torch.nn.functional as F

from .consts import *


device = "cuda" if torch.cuda.is_available() else "cpu"


class Model(nn.Module):
    def __init__(self) -> None:
        super().__init__()

        self.fc1 = nn.Linear(
            MODEL_IMAGE_WIDTH
            * (MODEL_TOP_IMAGE_LINES + MODEL_BOTTOM_IMAGE_LINES),
            128,
            device=device,
        )
        self.fc2 = nn.Linear(128, 64, device=device)
        self.fc3 = nn.Linear(64, 2, device=device)

    def forward(self, batch: torch.Tensor) -> torch.Tensor:
        with torch.autocast(device):
            x = F.leaky_relu(self.fc1(batch))
            x = F.leaky_relu(self.fc2(x))
            x = F.leaky_relu(self.fc3(x))

            return x

    def loss(self, output, expected) -> torch.Tensor:
        with torch.autocast(device):
            loss = nn.CrossEntropyLoss()(output, expected)

            return loss
