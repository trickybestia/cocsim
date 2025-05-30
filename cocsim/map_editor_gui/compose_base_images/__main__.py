from pathlib import Path
from sys import argv

from cocsim.consts import *

from . import compose_base_images, reverse_projection, load_base_images


def main():
    left, right = load_base_images(Path(TEST_IMAGES_PATH) / argv[1])

    composed = compose_base_images(left, right)
    square = reverse_projection(composed)

    square.show()


main()
