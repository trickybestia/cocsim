from io import BytesIO

import PIL
import PIL.Image
import PIL.ImageChops
import PIL.ImageDraw
import wand.image
import numpy as np

from cocsim.consts import *


SQUARE_IMAGE_SIZE = 832


def remove_vignette(
    image: PIL.Image.Image, strength: float = 0.26
) -> PIL.Image.Image:
    # AI GENERATED!!!

    # Open the image and convert to RGB (if not already in this mode)
    width, height = image.size

    # Convert to a numpy array (float32 for precision)
    img_array = np.array(image, dtype=np.float32) / 255.0

    # Create a normalized coordinate grid (ranging from -1 to 1)
    x = np.linspace(-1, 1, width)
    y = np.linspace(-1, 1, height)
    X, Y = np.meshgrid(x, y)

    # Vignette model (1 - strength * r^2)
    radius = np.sqrt(X**2 + Y**2)
    vignette = 1 - strength * (radius**2)

    # Normalize the vignette (so max value is 1)
    vignette = vignette / np.max(vignette)

    # Apply correction (divide the image by the vignette mask)
    corrected = (
        img_array / vignette[..., np.newaxis]
    )  # Add axis for RGB channels

    # Clip values and convert back to 0-255 range
    corrected = np.clip(corrected * 255, 0, 255).astype(np.uint8)

    # Create and save the corrected image
    result = PIL.Image.fromarray(corrected)

    return result


def compose_base_images(
    top: PIL.Image.Image, bottom: PIL.Image.Image
) -> PIL.Image.Image:
    bottom_paste_y = find_tear_line(top, bottom)

    composed = PIL.Image.new("RGB", (top.width, bottom_paste_y + bottom.height))

    composed.paste(top, (0, 0))
    composed.paste(bottom, (0, bottom_paste_y))

    return composed


def find_tear_line(top: PIL.Image.Image, bottom: PIL.Image.Image) -> int:
    IGNORE_BORDERS = 400
    MAX_HEIGHT = 100

    top = top.crop((IGNORE_BORDERS, 0, top.width - IGNORE_BORDERS, top.height))
    bottom = bottom.crop(
        (IGNORE_BORDERS, 0, bottom.width - IGNORE_BORDERS, bottom.height)
    )

    smallest_difference = None
    smallest_difference_y = None

    for y in range(top.height // 2, int(top.height * 0.8)):
        image = PIL.ImageChops.subtract(
            top.crop((0, y, top.width, y + MAX_HEIGHT)), bottom
        )
        difference = np.asarray(image, dtype="int32").sum() / image.size[1]

        if smallest_difference is None or difference < smallest_difference:
            smallest_difference_y = y
            smallest_difference = difference

        print(y, image.size, difference)

    print(smallest_difference_y, smallest_difference)

    return smallest_difference_y


def reverse_projection(image: PIL.Image.Image) -> PIL.Image.Image:
    TOP_CORNER_POS = (1220, 41)
    BOTTOM_CORNER_POS = (1221, 1536)
    LEFT_CORNER_POS = (222, 788)

    image_stream = BytesIO()

    image.save(image_stream, "bmp")

    with wand.image.Image(blob=image_stream.getvalue()) as wand_image:
        wand_image.artifacts["distort:viewport"] = (
            f"{SQUARE_IMAGE_SIZE}x{SQUARE_IMAGE_SIZE}+0+0"
        )
        wand_image.distort(
            "affine",
            (
                TOP_CORNER_POS[0],
                TOP_CORNER_POS[1],
                SQUARE_IMAGE_SIZE,
                0,
                BOTTOM_CORNER_POS[0],
                BOTTOM_CORNER_POS[1],
                0,
                SQUARE_IMAGE_SIZE,
                LEFT_CORNER_POS[0],
                LEFT_CORNER_POS[1],
                0,
                0,
            ),
        )

        return PIL.Image.open(BytesIO(wand_image.make_blob("bmp")))


def draw_grid(image: PIL.Image.Image):
    pixels_per_tile = SQUARE_IMAGE_SIZE // MAP_SIZE

    draw = PIL.ImageDraw.ImageDraw(image)

    for x in range(MAP_SIZE):
        for y in range(MAP_SIZE):
            draw.rectangle(
                (
                    (x * pixels_per_tile, y * pixels_per_tile),
                    (
                        (x + 1) * pixels_per_tile - 1,
                        (y + 1) * pixels_per_tile - 1,
                    ),
                ),
                outline="black",
            )


def main():
    for i in range(8):
        top = PIL.Image.open(f"test_images/top{i}.jpg")
        bottom = PIL.Image.open(f"test_images/bottom{i}.jpg")

        top = remove_vignette(top)
        bottom = remove_vignette(bottom)

        composed = compose_base_images(top, bottom)

        composed.show()

    return

    square = reverse_projection(composed)

    draw_grid(
        square,
    )

    square.show()


main()
