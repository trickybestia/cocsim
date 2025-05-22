from os import listdir
from random import randint

import torch
from torch.utils.data import DataLoader, TensorDataset
from safetensors.torch import load_model, save_model, save_file, load_file

import PIL.Image

from .model import Model, device, MODEL_PATH
from .logic import encode_image
from .consts import *


def add_image_to_dataset(
    image_path: str,
    all_inputs: list[torch.Tensor],
    all_outputs: list[torch.Tensor],
):
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

        all_inputs.append(torch.cat((top_inputs, bottom_inputs)))
        all_outputs.append(torch.tensor(outputs))

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

        all_inputs.append(torch.cat((top_inputs, bottom_inputs)))
        all_outputs.append(torch.tensor(outputs))


def load_dataset() -> tuple[torch.Tensor, torch.Tensor]:
    all_inputs = []
    all_outputs = []

    for image_path in listdir("compose_base_images_dataset"):
        if image_path.endswith(".jpg"):
            add_image_to_dataset(
                "compose_base_images_dataset/" + image_path,
                all_inputs,
                all_outputs,
            )

    return torch.stack(all_inputs), torch.stack(all_outputs)


def check_accuracy(model: Model) -> float:
    all_inputs: list[torch.Tensor] = []
    all_outputs: list[torch.Tensor] = []

    add_image_to_dataset(
        "compose_base_images_dataset/bottom0.jpg",
        all_inputs,
        all_outputs,
    )

    accuracy = 0

    for inputs, expected in zip(all_inputs, all_outputs):
        inputs = inputs.to(device)
        expected = expected.to(device)
        output = model.forward(inputs)

        if expected.argmax() == output.argmax():
            accuracy += 1

    return accuracy / len(all_inputs)


REBUILD_DATASET = True


def main():
    model = Model().to(device)

    load_model(model, MODEL_PATH, device=device)

    if REBUILD_DATASET:
        all_inputs, all_outputs = load_dataset()

        save_file(
            {"all_inputs": all_inputs, "all_outputs": all_outputs},
            "compose_base_images_dataset.safetensors",
        )
    else:
        loaded_dataset = load_file(
            "compose_base_images_dataset.safetensors", device=device
        )

        all_inputs = loaded_dataset["all_inputs"]
        all_outputs = loaded_dataset["all_outputs"]

    dataset = TensorDataset(all_inputs, all_outputs)
    loader = DataLoader(dataset, 512)

    optimizer = torch.optim.Adam(model.parameters(), lr=0.00001)

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

        if epoch % 50 == 0:
            save_model(model, MODEL_PATH)
            print("Model saved! Accuracy:", check_accuracy(model))


main()
