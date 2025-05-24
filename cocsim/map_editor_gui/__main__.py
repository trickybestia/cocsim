import PIL
import PIL.Image
import PIL.ImageDraw

from cocsim.consts import *
from .compose_base_images import (
    compose_4_base_images,
    reverse_projection,
    REVERSE_PROJECTION_IMAGE_SIZE,
)


def draw_grid(image: PIL.Image.Image):
    pixels_per_tile = REVERSE_PROJECTION_IMAGE_SIZE // MAP_SIZE

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

        composed = compose_4_base_images(
            left_top, left_bottom, right_top, right_bottom
        )

        square = reverse_projection(composed)

        draw_grid(
            square,
        )

        square.show()


main()
