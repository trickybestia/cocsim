import torch
import torch.nn as nn
import torch.nn.functional as F

from .consts import *


device = "cuda" if torch.cuda.is_available() else "cpu"


class Model(nn.Module):
    def __init__(self) -> None:
        super().__init__()

        self.conv1 = nn.Conv2d(1, 16, (3, 3), padding=1, device=device)
        self.conv2 = nn.Conv2d(16, 8, (3, 3), padding=1, device=device)
        self.fc1 = nn.Linear(
            8 * 20 * 40,
            128,
            device=device,
        )
        self.fc2 = nn.Linear(128, 64, device=device)
        self.fc3 = nn.Linear(64, 2, device=device)

    def forward(self, batch: torch.Tensor) -> torch.Tensor:
        with torch.autocast(device):
            x = F.relu(self.conv1(batch))
            x = F.dropout2d(F.max_pool2d(x, 2))
            x = F.relu(self.conv2(x))
            x = F.dropout2d(F.max_pool2d(x, 2))
            x = x.flatten(1)
            x = F.dropout(F.relu(self.fc1(x)))
            x = F.dropout(F.relu(self.fc2(x)))
            x = F.log_softmax(self.fc3(x), 1)

            return x

    def loss(
        self, output: torch.Tensor, expected_class: torch.Tensor
    ) -> torch.Tensor:
        with torch.autocast(device):
            loss = nn.NLLLoss()(output, expected_class)

            return loss
