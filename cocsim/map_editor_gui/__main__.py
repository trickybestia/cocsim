from io import BytesIO

import PIL
import PIL.Image
import PIL.ImageDraw
import wand.image

from cocsim.consts import *
from .compose_base_images import compose_base_images


SQUARE_IMAGE_SIZE = 832


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
    top = PIL.Image.open("compose_base_images_dataset/top1.jpg")
    bottom = PIL.Image.open("compose_base_images_dataset/bottom1.jpg")

    composed = compose_base_images(top, bottom)

    composed.show()  # first priority is to make NN work

    return

    square = reverse_projection(composed)

    draw_grid(
        square,
    )

    square.show()


main()
