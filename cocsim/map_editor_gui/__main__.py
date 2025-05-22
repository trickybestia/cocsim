from io import BytesIO

import PIL
import PIL.Image
import PIL.ImageChops
import PIL.ImageDraw
import wand.image
import numpy as np

from cocsim.consts import *


SQUARE_IMAGE_SIZE = 832


def remove_vignette(image: PIL.Image.Image):
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


def compose_base_images(
    top: PIL.Image.Image, bottom: PIL.Image.Image
) -> PIL.Image.Image:
    BOTTOM_CROP_Y = 0  # vignette end

    remove_vignette(bottom)

    bottom = bottom.crop((0, BOTTOM_CROP_Y, bottom.width, bottom.height))

    bottom_paste_y = find_tear_line(top, bottom) + 1

    composed = PIL.Image.new(
        "RGB", (top.width, bottom_paste_y - BOTTOM_CROP_Y + bottom.height)
    )

    composed.paste(top, (0, 0))
    composed.paste(bottom, (0, bottom_paste_y))

    return composed


def find_tear_line(top: PIL.Image.Image, bottom: PIL.Image.Image) -> int:
    smallest_difference = None
    smallest_difference_y = None

    for y in range(top.height // 2, int(top.height * 0.8)):
        image = PIL.ImageChops.subtract(
            top.crop((0, y, top.width, top.height)), bottom
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
    top = PIL.Image.open("compose_base_images_dataset/top2.jpg")
    bottom = PIL.Image.open("compose_base_images_dataset/bottom2.jpg")

    composed = compose_base_images(top, bottom)

    composed.show()  # first priority is to make NN work

    return

    square = reverse_projection(composed)

    draw_grid(
        square,
    )

    square.show()


main()
