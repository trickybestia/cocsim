from io import BytesIO

import PIL
import PIL.Image
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


def _compose_base_images_internal(
    top: PIL.Image.Image,
    bottom: PIL.Image.Image,
    ignore_borders: int,
    y_start: int,
    y_stop: int,
) -> PIL.Image.Image:
    bottom_paste_y = _find_tear_line(
        top, bottom, ignore_borders, y_start, y_stop
    )

    composed = PIL.Image.new("RGB", (top.width, bottom_paste_y + bottom.height))

    composed.paste(top, (0, 0))
    composed.paste(bottom, (0, bottom_paste_y))

    return composed


def _find_tear_line(
    top: PIL.Image.Image,
    bottom: PIL.Image.Image,
    ignore_borders: int,
    y_start: int,
    y_stop: int,
) -> int:
    WINDOW_HEIGHT = 100

    top = top.crop((ignore_borders, 0, top.width - ignore_borders, top.height))
    bottom = bottom.crop(
        (ignore_borders, 0, bottom.width - ignore_borders, bottom.height)
    )

    smallest_difference = None
    smallest_difference_y = None

    for y in range(y_start, min(y_stop, top.height - WINDOW_HEIGHT)):
        top_crop = top.crop((0, y, top.width, y + WINDOW_HEIGHT))
        bottom_crop = bottom.crop((0, 0, bottom.width, WINDOW_HEIGHT))
        difference = np.abs(
            np.asarray(top_crop, dtype="int32")
            - np.asarray(bottom_crop, dtype="int32")
        ).sum()

        if smallest_difference is None or difference < smallest_difference:
            smallest_difference_y = y
            smallest_difference = difference

    return smallest_difference_y


def compose_4_base_images(
    left_top: PIL.Image.Image,
    left_bottom: PIL.Image.Image,
    right_top: PIL.Image.Image,
    right_bottom: PIL.Image.Image,
) -> PIL.Image.Image:
    VERTICAL_IGNORE_BORDERS = 400
    VERTICAL_Y_START = 400
    VERTICAL_Y_STOP = 900

    left = _compose_base_images_internal(
        left_top,
        left_bottom,
        VERTICAL_IGNORE_BORDERS,
        VERTICAL_Y_START,
        VERTICAL_Y_STOP,
    )
    right = _compose_base_images_internal(
        right_top,
        right_bottom,
        VERTICAL_IGNORE_BORDERS,
        VERTICAL_Y_START,
        VERTICAL_Y_STOP,
    )

    left = left.crop((0, 0, left.width // 2, left.height))

    HORIZONTAL_IGNORE_BORDERS = 150
    HORIZONTAL_Y_START = 200
    HORIZONTAL_Y_STOP = 1500

    composed = _compose_base_images_internal(
        right.rotate(90, expand=True),
        left.rotate(90, expand=True),
        HORIZONTAL_IGNORE_BORDERS,
        HORIZONTAL_Y_START,
        HORIZONTAL_Y_STOP,
    ).rotate(-90, expand=True)

    return composed


def reverse_projection(image: PIL.Image.Image) -> PIL.Image.Image:
    TOP_CORNER_POS = (1250, 41)
    BOTTOM_CORNER_POS = (1247, 1572)
    LEFT_CORNER_POS = (223, 810)

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
    for i in range(4):
        left_top = PIL.Image.open(f"test_images/lt{i}.jpg")
        left_bottom = PIL.Image.open(f"test_images/lb{i}.jpg")
        right_top = PIL.Image.open(f"test_images/rt{i}.jpg")
        right_bottom = PIL.Image.open(f"test_images/rb{i}.jpg")

        left_top = remove_vignette(left_top)
        left_bottom = remove_vignette(left_bottom)
        right_top = remove_vignette(right_top)
        right_bottom = remove_vignette(right_bottom)

        composed = compose_4_base_images(
            left_top, left_bottom, right_top, right_bottom
        ).resize((2498, 1756))

        square = reverse_projection(composed)

        draw_grid(
            square,
        )

        square.show()


main()
