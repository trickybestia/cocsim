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
    image_path: str, samples: int
) -> Generator[tuple[torch.Tensor, torch.Tensor], None, None]:
    image = PIL.Image.open(image_path).convert("L")

    samples //= 2

    for _ in range(samples):
        y = randint(
            MODEL_IMAGE_CONTEXT_LINES - 1,
            image.height - MODEL_IMAGE_CONTEXT_LINES - 1,
        )
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

        yield torch.cat((top_inputs, bottom_inputs)), torch.tensor(1)

    for _ in range(samples):
        y_top = randint(0, image.height - MODEL_IMAGE_CONTEXT_LINES - 1)
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

        yield torch.cat((top_inputs, bottom_inputs)), torch.tensor(0)


def create_dataset(source_path: str, destination_path: str):
    os.mkdir(destination_path)

    i = 0

    for image_name in os.listdir(source_path):
        if image_name.endswith(".jpg"):
            for inputs, outputs in add_image_to_dataset(
                f"{source_path}/{image_name}", DATASET_SAMPLES_PER_IMAGE
            ):
                save_file(
                    {"inputs": inputs, "output_class": outputs},
                    f"{destination_path}/{i}.safetensors",
                )

                i += 1


def check_accuracy(model: Model) -> float:
    accuracy = 0
    count = 0

    with torch.no_grad():
        for inputs, expected_class in add_image_to_dataset(
            f"test_images/top1.jpg", DATASET_SAMPLES_PER_IMAGE
        ):
            inputs = inputs.to(device).reshape((1, -1))
            expected_class = expected_class.to(device)
            output = model.forward(inputs)[0]

            if output.argmax() == expected_class.item():
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

        return loaded_dict["inputs"], loaded_dict["output_class"]

    def __len__(self) -> int:
        return len(os.listdir(self.directory_path))


def main():
    model = Model().to(device)

    if not os.path.exists(DATASET_PATH):
        create_dataset(RAW_DATASET_PATH, DATASET_PATH)

    if os.path.exists(MODEL_PATH):
        load_model(model, MODEL_PATH, device=device)

    dataset = ComposeBaseImagesDataset(DATASET_PATH)
    loader = DataLoader(dataset, BATCH_SIZE, shuffle=True, num_workers=1)

    optimizer = torch.optim.Adam(model.parameters(), lr=0.0001)

    for epoch in range(1000):
        epoch_loss = 0

        for inputs, expected_class in loader:
            inputs = inputs.to(device)
            expected_class = expected_class.to(device)

            output = model.forward(inputs)

            model.zero_grad()
            loss = model.loss(output, expected_class)
            loss.backward()
            optimizer.step()

            epoch_loss += loss.item()

        print(f"{epoch}: loss={epoch_loss}")

        save_model(model, MODEL_PATH)
        print("Model saved! Accuracy:", check_accuracy(model))


main()
