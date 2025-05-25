import json
from pathlib import Path

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

    map_path = Path("test_maps") / "practice_giant_smash.json"

    if map_path.exists():
        map_buildings = json.loads(map_path.read_text())
    else:
        map_buildings = []

    window = MainWindow(base_image, map_buildings)

    window.run()

    map_path.write_text(json.dumps(window.get_map_buildings(), indent=4) + "\n")


main()
