from io import BytesIO

import PIL
import PIL.Image
import wand.image
import numpy as np

from cocsim.consts import *


def compose_base_images(
    left: list[PIL.Image.Image],
    right: list[PIL.Image.Image],
) -> PIL.Image.Image:
    for i in range(len(left)):
        left[i] = _remove_vignette(left[i])

    for i in range(len(right)):
        right[i] = _remove_vignette(right[i])

    VERTICAL_IGNORE_BORDERS = 400
    Y_SKIP_FIRST = 200
    Y_SKIP_LAST = 200

    left_composed = left[0]

    for i in range(1, len(left)):
        left_composed = _compose_base_images_internal(
            left_composed,
            left[i],
            VERTICAL_IGNORE_BORDERS,
            Y_SKIP_FIRST,
            Y_SKIP_LAST,
        )

    right_composed = right[0]

    for i in range(1, len(right)):
        right_composed = _compose_base_images_internal(
            right_composed,
            right[i],
            VERTICAL_IGNORE_BORDERS,
            Y_SKIP_FIRST,
            Y_SKIP_LAST,
        )

    left_composed = left_composed.crop(
        (0, 0, left_composed.width // 2, left_composed.height)
    )

    HORIZONTAL_IGNORE_BORDERS = 150
    X_SKIP_FIRST = 400
    X_SKIP_LAST = 400

    composed = _compose_base_images_internal(
        right_composed.rotate(90, expand=True),
        left_composed.rotate(90, expand=True),
        HORIZONTAL_IGNORE_BORDERS,
        X_SKIP_FIRST,
        X_SKIP_LAST,
    ).rotate(-90, expand=True)

    return composed


def reverse_projection(image: PIL.Image.Image) -> PIL.Image.Image:
    RESIZE_WIDTH = 2498
    RESIZE_HEIGHT = 1756
    RESIZE_ASPECT_RATIO = RESIZE_WIDTH / RESIZE_HEIGHT

    aspect_ratio = image.width / image.height

    if aspect_ratio > RESIZE_ASPECT_RATIO:
        image = image.crop(
            (0, 0, round(image.height * RESIZE_ASPECT_RATIO), image.height)
        )
    else:
        image = image.crop(
            (0, 0, image.width, round(image.width / RESIZE_ASPECT_RATIO))
        )

    TOP_CORNER_POS = (1250, 41)
    BOTTOM_CORNER_POS = (1247, 1572)
    LEFT_CORNER_POS = (223, 810)

    image_stream = BytesIO()

    image.resize((RESIZE_WIDTH, RESIZE_HEIGHT)).save(image_stream, "bmp")

    with wand.image.Image(blob=image_stream.getvalue()) as wand_image:
        wand_image.artifacts["distort:viewport"] = f"1800x1800+0+0"
        wand_image.distort(
            "affine",
            (
                TOP_CORNER_POS[0],
                TOP_CORNER_POS[1],
                1500,
                300,
                BOTTOM_CORNER_POS[0],
                BOTTOM_CORNER_POS[1],
                300,
                1500,
                LEFT_CORNER_POS[0],
                LEFT_CORNER_POS[1],
                300,
                300,
            ),
        )

        return PIL.Image.open(BytesIO(wand_image.make_blob("bmp")))


def _remove_vignette(
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
    y_skip_first: int,
    y_skip_last: int,
) -> PIL.Image.Image:
    bottom_paste_y = _find_tear_line(
        top, bottom, ignore_borders, y_skip_first, top.height - y_skip_last
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
