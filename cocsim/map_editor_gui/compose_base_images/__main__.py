import os
from random import randint
from typing import Generator

import torch
from torch.utils.data import DataLoader, Dataset
from safetensors.torch import load_model, save_model, save_file, load_file
import PIL.Image

from .model import Model, device, MODEL_PATH
from .logic import encode_image, remove_top_vignette
from .consts import *


def add_image_to_dataset(
    image_path: str, samples: int
) -> Generator[tuple[torch.Tensor, torch.Tensor], None, None]:
    image = PIL.Image.open(image_path)

    remove_top_vignette(image)

    image = image.convert("L")

    samples //= 2

    for _ in range(samples):
        x = randint(
            IGNORE_BORDERS_X,
            image.width - MODEL_IMAGE_SIZE[0] - IGNORE_BORDERS_X - 1,
        )
        y_top = randint(0, image.height - 2 * MODEL_IMAGE_SIZE[1] - 1)
        y_bottom = y_top + MODEL_IMAGE_SIZE[1]

        top_input_image = image.crop(
            (
                x,
                y_top,
                x + MODEL_IMAGE_SIZE[0],
                y_top + MODEL_IMAGE_SIZE[1],
            )
        )
        top_inputs = encode_image(top_input_image)
        bottom_input_image = image.crop(
            (
                x,
                y_bottom,
                x + MODEL_IMAGE_SIZE[0],
                y_bottom + MODEL_IMAGE_SIZE[1],
            )
        )
        bottom_inputs = encode_image(bottom_input_image)

        yield torch.cat((top_inputs, bottom_inputs)), torch.tensor(1)

    for _ in range(samples):
        x = randint(
            IGNORE_BORDERS_X,
            image.width - MODEL_IMAGE_SIZE[0] - IGNORE_BORDERS_X - 1,
        )
        y_top = randint(0, image.height - MODEL_IMAGE_SIZE[1] - 1)
        y_bottom = randint(0, image.height - MODEL_IMAGE_SIZE[1] - 1)

        if y_top == y_bottom or y_bottom == y_top + MODEL_IMAGE_SIZE[1]:
            continue

        top_input_image = image.crop(
            (
                x,
                y_top,
                x + MODEL_IMAGE_SIZE[0],
                y_top + MODEL_IMAGE_SIZE[1],
            )
        )
        top_inputs = encode_image(top_input_image)
        bottom_input_image = image.crop(
            (
                x,
                y_bottom,
                x + MODEL_IMAGE_SIZE[0],
                y_bottom + MODEL_IMAGE_SIZE[1],
            )
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

    model.train(False)

    with torch.no_grad():
        for inputs, expected_class in add_image_to_dataset(
            f"test_images/top1.jpg", DATASET_SAMPLES_PER_IMAGE
        ):
            inputs = inputs.to(device).unsqueeze(0)
            expected_class = expected_class.to(device)
            output = model.forward(inputs)[0]

            if output.argmax() == expected_class.item():
                accuracy += 1

            count += 1

    model.train(True)

    return accuracy / count


class ComposeBaseImagesDataset(Dataset):
    _directory_path: str
    _length: int | None

    def __init__(self, directory_path: str):
        super().__init__()

        self._directory_path = directory_path
        self._length = None

    def __getitem__(self, index: int):
        loaded_dict = load_file(f"{self._directory_path}/{index}.safetensors")

        return loaded_dict["inputs"], loaded_dict["output_class"]

    def __len__(self) -> int:
        if self._length is None:
            self._length = len(os.listdir(self._directory_path))

        return self._length


def main():
    model = Model().to(device)

    if not os.path.exists(DATASET_PATH):
        create_dataset(RAW_DATASET_PATH, DATASET_PATH)

    if os.path.exists(MODEL_PATH):
        load_model(model, MODEL_PATH, device=device)

    dataset = ComposeBaseImagesDataset(DATASET_PATH)
    loader = DataLoader(dataset, BATCH_SIZE, shuffle=True, num_workers=1)

    optimizer = torch.optim.Adam(model.parameters())
    scheduler = torch.optim.lr_scheduler.ReduceLROnPlateau(
        optimizer, threshold=0.01, threshold_mode="abs"
    )

    for epoch in range(1000):
        epoch_loss = 0
        epoch_accuracy = 0
        batch_count = 0
        samples_count = 0

        for inputs, expected_class in loader:
            inputs = inputs.to(device)
            expected_class = expected_class.to(device)

            output = model.forward(inputs)

            model.zero_grad()
            loss = model.loss(output, expected_class)
            loss.backward()
            optimizer.step()

            epoch_loss += loss.item()
            epoch_accuracy += (output.argmax(1) == expected_class).sum().item()
            batch_count += 1
            samples_count += inputs.shape[0]

        epoch_loss /= batch_count
        epoch_accuracy /= samples_count

        print(
            f"{epoch}: loss={epoch_loss}, accuracy={epoch_accuracy}, lr={optimizer.param_groups[0]['lr']}"
        )

        scheduler.step(epoch_loss)

        if epoch % 5 == 0:
            save_model(model, MODEL_PATH)
            print("Model saved! Test dataset accuracy:", check_accuracy(model))


main()
