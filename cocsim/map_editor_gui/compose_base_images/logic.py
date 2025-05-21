import PIL.Image
from safetensors.torch import load_model

from .model import Model, device, MODEL_PATH


model: Model | None = None


def compose_base_images(
    top: PIL.Image.Image, bottom: PIL.Image.Image
) -> PIL.Image.Image:
    BOTTOM_CROP_Y = 200  # vignette end

    bottom = bottom.crop((0, BOTTOM_CROP_Y, bottom.width, bottom.height))

    bottom_paste_y = _find_tear_line(top, bottom) + 1 - 200

    composed = PIL.Image.new(
        "RGB", (top.width, bottom_paste_y - BOTTOM_CROP_Y + bottom.height)
    )

    composed.paste(top, (0, 0))
    composed.paste(bottom, (0, bottom_paste_y))

    return composed


def _find_tear_line(top: PIL.Image.Image, bottom: PIL.Image.Image) -> int:
    global model

    if model is None:
        model = Model().to(device)

        load_model(model, MODEL_PATH)

    top_grayscale = top.convert("L")
    bottom_grayscale = bottom.convert("L")

    data = [0] * 2 * bottom.width

    for x in range(bottom.width):
        data[x + bottom.width] = bottom_grayscale.getpixel((x, 0))

    last_y = None

    # for y in range(top.height // 2, int(top.height * 0.8)):
    for y in range(top.height):
        for x in range(bottom.width):
            data[x] = top_grayscale.getpixel((x, y))

        if model.inference_forward(data):
            print(y)

            last_y = y

    return last_y
