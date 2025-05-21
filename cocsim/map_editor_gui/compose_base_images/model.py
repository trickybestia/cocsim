import torch
import torch.nn as nn
import torch.nn.functional as F

MODEL_PATH = "model.safetensors"


device = "cuda" if torch.cuda.is_available() else "cpu"


class Model(nn.Module):
    def __init__(self) -> None:
        super().__init__()

        self.fc1 = nn.Linear(2400 * 2, 128, device=device)
        self.fc2 = nn.Linear(128, 384, device=device)
        self.fc3 = nn.Linear(384, 2, device=device)

    def inference_forward(self, data: list[int]) -> bool:
        with torch.no_grad():
            inputs = torch.tensor(data) / 255 - 0.5

            return self.forward(inputs).argmax().item() == 0

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
