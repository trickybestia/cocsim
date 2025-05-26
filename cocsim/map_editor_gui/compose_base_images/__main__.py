from sys import argv

from . import compose_base_images, reverse_projection, load_test_images


def main():
    left, right = load_test_images(int(argv[1]))

    composed = compose_base_images(left, right)
    square = reverse_projection(composed)

    square.show()


main()
