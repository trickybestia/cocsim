import PIL
import PIL.Image


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


def concat_base_images(
    top: PIL.Image.Image, bottom: PIL.Image.Image
) -> PIL.Image.Image:
    BOTTOM_PASTE_Y = 636

    remove_vignette(bottom)

    composed = PIL.Image.new("RGB", (top.width, BOTTOM_PASTE_Y + bottom.height))

    composed.paste(top, (0, 0))
    composed.paste(bottom, (0, BOTTOM_PASTE_Y))

    return composed


def main():
    top = PIL.Image.open("top.jpg")
    bottom = PIL.Image.open("bottom.jpg")

    concat_base_images(top, bottom).show()


main()
