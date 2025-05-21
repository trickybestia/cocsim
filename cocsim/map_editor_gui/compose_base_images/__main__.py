from os import listdir
from random import randint

import torch
from torch.utils.data import DataLoader, TensorDataset
from safetensors.torch import load_model, save_model, save_file, load_file

import PIL.Image

from .model import Model, device, MODEL_PATH


def add_image_to_dataset(
    image_path: str,
    all_inputs: list[torch.Tensor],
    all_outputs: list[torch.Tensor],
    items: int | None = None,
):
    image = PIL.Image.open(image_path).convert("L")

    if items is None:
        items = image.height
    else:
        items //= 2

    for y in range(1, image.height):
        inputs = []
        outputs = [1.0, 0.0]

        for x in range(image.width):
            inputs.append(image.getpixel((x, y - 1)))
        for x in range(image.width):
            inputs.append(image.getpixel((x, y)))

        all_inputs.append(torch.tensor(inputs) / 255 - 0.5)
        all_outputs.append(torch.tensor(outputs))

    for y in range(image.height):
        inputs = []
        outputs = [0.0, 1.0]

        y1 = randint(0, image.height - 1)

        if y == y1 or y1 == y - 1:
            continue

        for x in range(image.width):
            inputs.append(image.getpixel((x, y)))
        for x in range(image.width):
            inputs.append(image.getpixel((x, y1)))

        all_inputs.append(torch.tensor(inputs) / 255 - 0.5)
        all_outputs.append(torch.tensor(outputs))


def load_dataset() -> tuple[torch.Tensor, torch.Tensor]:
    all_inputs = []
    all_outputs = []

    for image_path in listdir("compose_base_images_dataset"):
        add_image_to_dataset(
            "compose_base_images_dataset/" + image_path,
            all_inputs,
            all_outputs,
            100,
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
        output = model.forward(inputs)

        if expected.argmax() == output.argmax():
            accuracy += 1

    return accuracy / len(all_inputs)


REBUILD_DATASET = True


def main():
    model = Model().to(device)

    load_model(model, MODEL_PATH)

    # print(check_accuracy(model))

    # return

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
    loader = DataLoader(dataset, 32, num_workers=1)

    optimizer = torch.optim.Adam(model.parameters())

    for epoch in range(1000):
        epoch_loss = 0

        for i, batch in enumerate(loader):
            inputs = batch[0]
            expected = batch[1]

            output = model.forward(inputs)

            model.zero_grad()
            loss = model.loss(output, expected)
            loss.backward()
            optimizer.step()

            epoch_loss += loss.item()

        print(f"{epoch}: loss={epoch_loss}")

        if epoch % 10 == 0:
            save_model(model, MODEL_PATH)
            print("Model saved! Accuracy:", check_accuracy(model))


main()
