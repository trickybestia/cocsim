import torch
import PIL.Image
from safetensors.torch import load_model

from .model import Model, device
from .consts import *


model: Model | None = None


def compose_base_images(
    top: PIL.Image.Image, bottom: PIL.Image.Image
) -> PIL.Image.Image:
    BOTTOM_CROP_Y = 200  # vignette end

    bottom = bottom.crop((0, BOTTOM_CROP_Y, bottom.width, bottom.height))

    bottom_paste_y = _find_tear_line(top, bottom) + 1

    composed = PIL.Image.new("RGB", (top.width, bottom_paste_y + bottom.height))

    composed.paste(top, (0, 0))
    composed.paste(bottom, (0, bottom_paste_y))

    return composed


def encode_image(image: PIL.Image.Image) -> torch.Tensor:
    return (
        torch.frombuffer(
            bytearray(image.convert("L").resize(MODEL_IMAGE_RESIZE).tobytes()),
            dtype=torch.uint8,
        ).to(dtype=torch.float)
        / 255
        - 0.5
    )


def _find_tear_line(top: PIL.Image.Image, bottom: PIL.Image.Image) -> int:
    global model

    if model is None:
        model = Model().to(device)

        load_model(model, MODEL_PATH, device=device)

        model.eval()

    top_grayscale = top.convert("L")
    bottom_grayscale = bottom.convert("L")

    bottom_input_image = bottom_grayscale.crop(
        (0, 0, top.width, MODEL_IMAGE_CONTEXT_LINES)
    )
    bottom_inputs = encode_image(bottom_input_image).to(device=device)

    max_certainity_y = None
    max_certainity = None

    # for y in range(top.height // 2, int(top.height * 0.8)):
    for y in range(MODEL_IMAGE_CONTEXT_LINES - 1, top.height):
        top_input_image = top_grayscale.crop(
            (
                0,
                y - MODEL_IMAGE_CONTEXT_LINES + 1,
                top.width,
                y + 1,
            )
        )
        top_inputs = encode_image(top_input_image).to(device=device)
        inputs = torch.cat((top_inputs, bottom_inputs))

        with torch.no_grad():
            outputs = model.forward(inputs).exp()
            certainity = outputs.max().item()

            if outputs.argmax().item() == 0:
                print(y, certainity)

                if max_certainity is None or certainity > max_certainity:
                    max_certainity_y = y
                    max_certainity = certainity

    print(max_certainity_y)

    return max_certainity_y
