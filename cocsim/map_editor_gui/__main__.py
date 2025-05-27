import json
from pathlib import Path
from sys import argv

from cocsim.consts import *
from .main_window import MainWindow
from .compose_base_images import (
    compose_base_images,
    reverse_projection,
    load_base_images,
)


def main():
    left, right = load_base_images(Path("test_images") / argv[1])
    composed = compose_base_images(left, right)
    base_image = reverse_projection(composed)

    window = MainWindow(base_image)

    map_path = Path("test_maps") / "single_player_goblin_gauntlet.json"

    if map_path.exists():
        map = json.loads(map_path.read_text())

        window.set_map(map)

    window.run()

    map_path.write_text(json.dumps(window.get_map(), indent=4) + "\n")


main()
