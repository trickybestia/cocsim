import PIL
import PIL.Image


def concat_base_images(
    top: PIL.Image.Image, bottom: PIL.Image.Image
) -> PIL.Image.Image:
    BOTTOM_CROP_Y = 236
    BOTTOM_PASTE_Y = 636 + BOTTOM_CROP_Y

    composed = PIL.Image.new(
        "RGB", (top.width, BOTTOM_PASTE_Y - BOTTOM_CROP_Y + bottom.height)
    )

    bottom = bottom.crop((0, BOTTOM_CROP_Y, bottom.width, bottom.height))

    composed.paste(top, (0, 0))
    composed.paste(bottom, (0, BOTTOM_PASTE_Y))

    return composed


def main():
    top = PIL.Image.open("top.jpg")
    bottom = PIL.Image.open("bottom.jpg")

    concat_base_images(top, bottom).show()


main()
