import torch
import torch.nn as nn
import torch.nn.functional as F

from .consts import *


device = "cuda" if torch.cuda.is_available() else "cpu"


class Model(nn.Module):
    def __init__(self) -> None:
        super().__init__()

        self.fc1 = nn.Linear(
            MODEL_IMAGE_RESIZE[0] * MODEL_IMAGE_RESIZE[1] * 2,
            128,
            device=device,
        )
        self.fc2 = nn.Linear(128, 64, device=device)
        self.fc3 = nn.Linear(64, 2, device=device)

    def forward(self, batch: torch.Tensor) -> torch.Tensor:
        with torch.autocast(device):
            x = F.relu(self.fc1(batch))
            x = F.relu(self.fc2(x))
            x = self.fc3(x)
            x = F.log_softmax(x, 1)

            return x

    def loss(
        self, output: torch.Tensor, expected_class: torch.Tensor
    ) -> torch.Tensor:
        with torch.autocast(device):
            loss = nn.NLLLoss()(output, expected_class)

            return loss
