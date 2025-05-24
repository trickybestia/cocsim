import PIL
import PIL.Image

from cocsim.consts import *
from .main_window import MainWindow
from .compose_base_images import compose_4_base_images, reverse_projection


def main():
    i = 2
    left_top = PIL.Image.open(f"test_images/lt{i}.jpg")
    left_bottom = PIL.Image.open(f"test_images/lb{i}.jpg")
    right_top = PIL.Image.open(f"test_images/rt{i}.jpg")
    right_bottom = PIL.Image.open(f"test_images/rb{i}.jpg")

    composed = compose_4_base_images(
        left_top, left_bottom, right_top, right_bottom
    )

    base_image = reverse_projection(composed)

    MainWindow(base_image).run()


main()
