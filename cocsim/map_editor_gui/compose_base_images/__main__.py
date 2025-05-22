import os
from random import randint
from typing import Generator

import torch
from torch.utils.data import DataLoader, Dataset
from safetensors.torch import load_model, save_model, save_file, load_file
import PIL.Image

from .model import Model, device, MODEL_PATH
from .logic import encode_image
from .consts import *


def add_image_to_dataset(
    image_path: str,
) -> Generator[tuple[torch.Tensor, torch.Tensor], None, None]:
    image = PIL.Image.open(image_path).convert("L")

    for y in range(
        MODEL_IMAGE_CONTEXT_LINES - 1, image.height - MODEL_IMAGE_CONTEXT_LINES
    ):
        outputs = [1.0, 0.0]

        top_input_image = image.crop(
            (
                0,
                y - MODEL_IMAGE_CONTEXT_LINES + 1,
                image.width,
                y + 1,
            )
        )
        top_inputs = encode_image(top_input_image)
        bottom_input_image = image.crop(
            (0, y + 1, image.width, y + 1 + MODEL_IMAGE_CONTEXT_LINES)
        )
        bottom_inputs = encode_image(bottom_input_image)

        yield torch.cat((top_inputs, bottom_inputs)), torch.tensor(outputs)

    for y_top in range(image.height - MODEL_IMAGE_CONTEXT_LINES):
        outputs = [0.0, 1.0]

        y_bottom = randint(0, image.height - MODEL_IMAGE_CONTEXT_LINES - 1)

        if y_top == y_bottom or y_bottom == y_top + MODEL_IMAGE_CONTEXT_LINES:
            continue

        top_input_image = image.crop(
            (
                0,
                y_top,
                image.width,
                y_top + MODEL_IMAGE_CONTEXT_LINES,
            )
        )
        top_inputs = encode_image(top_input_image)
        bottom_input_image = image.crop(
            (0, y_bottom, image.width, y_bottom + MODEL_IMAGE_CONTEXT_LINES)
        )
        bottom_inputs = encode_image(bottom_input_image)

        yield torch.cat((top_inputs, bottom_inputs)), torch.tensor(outputs)


def create_dataset(source_path: str, destination_path: str):
    os.mkdir(destination_path)

    i = 0

    for image_name in os.listdir(source_path):
        if image_name.endswith(".jpg"):
            for inputs, outputs in add_image_to_dataset(
                f"{source_path}/{image_name}"
            ):
                save_file(
                    {"inputs": inputs, "outputs": outputs},
                    f"{destination_path}/{i}.safetensors",
                )

                i += 1


def check_accuracy(model: Model) -> float:
    accuracy = 0
    count = 0

    with torch.no_grad():
        for inputs, expected in add_image_to_dataset(
            f"{RAW_DATASET_PATH}/Screenshot_2025-05-22-07-41-50-220_com.supercell.clashofclans.jpg"
        ):
            inputs = inputs.to(device)
            expected = expected.to(device)
            output = model.forward(inputs)

            if expected.argmax() == output.argmax():
                accuracy += 1

            count += 1

    return accuracy / count


class ComposeBaseImagesDataset(Dataset):
    directory_path: str

    def __init__(self, directory_path: str):
        super().__init__()

        self.directory_path = directory_path

    def __getitem__(self, index: int):
        loaded_dict = load_file(f"{self.directory_path}/{index}.safetensors")

        return loaded_dict["inputs"], loaded_dict["outputs"]

    def __len__(self) -> int:
        return len(os.listdir(self.directory_path))


def main():
    model = Model().to(device)

    if RECREATE_DATASET:
        create_dataset(RAW_DATASET_PATH, DATASET_PATH)

    load_model(model, MODEL_PATH, device=device)

    dataset = ComposeBaseImagesDataset(DATASET_PATH)
    loader = DataLoader(dataset, 32, shuffle=True, num_workers=1)

    optimizer = torch.optim.Adam(model.parameters())

    for epoch in range(1000):
        epoch_loss = 0

        for inputs, expected in loader:
            inputs = inputs.to(device)
            expected = expected.to(device)

            output = model.forward(inputs)

            model.zero_grad()
            loss = model.loss(output, expected)
            loss.backward()
            optimizer.step()

            epoch_loss += loss.item()

        print(f"{epoch}: loss={epoch_loss}")

        save_model(model, MODEL_PATH)
        print("Model saved! Accuracy:", check_accuracy(model))


main()
