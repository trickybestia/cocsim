import torch
import PIL.Image
from safetensors.torch import load_model

from .model import Model, device
from .consts import *


model: Model | None = None


def compose_base_images(
    top: PIL.Image.Image, bottom: PIL.Image.Image
) -> PIL.Image.Image:
    remove_top_vignette(bottom)

    bottom_paste_y = _find_tear_line(top, bottom) + 1

    composed = PIL.Image.new("RGB", (top.width, bottom_paste_y + bottom.height))

    composed.paste(top, (0, 0))
    composed.paste(bottom, (0, bottom_paste_y))

    return composed


def encode_image(image: PIL.Image.Image) -> torch.Tensor:
    return (
        torch.frombuffer(
            bytearray(image.convert("L").tobytes()),
            dtype=torch.uint8,
        ).to(dtype=torch.float)
        / 127.5
        - 1.0
    )


def remove_top_vignette(image: PIL.Image.Image):
    VIGNETTE_SIZE = 200

    for y in range(VIGNETTE_SIZE):
        r_offset = (
            9.340e-06 * y**3 - 2.450e-03 * y**2 - 4.349e-02 * y + 3.862e01
        )
        g_offset = (
            1.106e-05 * y**3 - 3.010e-03 * y**2 - 3.807e-02 * y + 4.894e01
        )
        b_offset = (
            3.450e-06 * y**3 - 1.006e-03 * y**2 - 2.616e-02 * y + 2.091e01
        )

        if y >= 180:
            K = (200 - y) / (200 - 180)
            r_offset *= K
            g_offset *= K
            b_offset *= K

        r_offset = int(r_offset)
        g_offset = int(g_offset)
        b_offset = int(b_offset)

        for x in range(image.width):
            r, g, b = image.getpixel((x, y))

            image.putpixel(
                (x, y),
                (
                    r + r_offset,
                    g + g_offset,
                    b + b_offset,
                ),
            )


def _find_tear_line(top: PIL.Image.Image, bottom: PIL.Image.Image) -> int:
    global model

    if model is None:
        model = Model().to(device)

        load_model(model, MODEL_PATH, device=device)

        model.eval()

    top_grayscale = top.convert("L")
    bottom_grayscale = bottom.convert("L")

    max_certainity_y = None
    max_certainity = None

    # for y in range(top.height // 2, int(top.height * 0.8)):
    for y in range(top.height - MODEL_IMAGE_SIZE[1]):
        certainity = 0

        for x in range(
            IGNORE_BORDERS_X,
            top.width - MODEL_IMAGE_SIZE[0] - 1 - IGNORE_BORDERS_X,
            MODEL_IMAGE_SIZE[0],
        ):
            bottom_input_image = bottom_grayscale.crop(
                (x, 0, x + MODEL_IMAGE_SIZE[0], MODEL_IMAGE_SIZE[1])
            )
            bottom_inputs = encode_image(bottom_input_image).to(device=device)
            top_input_image = top_grayscale.crop(
                (
                    x,
                    y - MODEL_IMAGE_SIZE[1] + 1,
                    x + MODEL_IMAGE_SIZE[0],
                    y + 1,
                )
            )
            top_inputs = encode_image(top_input_image).to(device=device)
            inputs = torch.cat((top_inputs, bottom_inputs)).unsqueeze(0)

            with torch.no_grad():
                outputs = model.forward(inputs).exp()[0]

                if outputs.argmax().item() == 1:
                    certainity += 1

        if max_certainity is None or certainity > max_certainity:
            max_certainity_y = y
            max_certainity = certainity

        print(y, certainity)

    print(max_certainity_y)

    return max_certainity_y
