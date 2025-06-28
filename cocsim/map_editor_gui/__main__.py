import json
from pathlib import Path
from sys import argv

import PIL.Image

from cocsim.consts import *
from cocsim.map import Map

from .compose_base_images import (
    compose_base_images,
    load_base_images,
    reverse_projection,
)
from .main_window import MainWindow


def get_map(name: str) -> tuple[Map | None, PIL.Image.Image]:
    map_dir_path = Path(TEST_MAPS_PATH) / name

    if map_dir_path.exists():
        map_path = map_dir_path / "map.json"
        map_image_path = map_dir_path / "map.jpg"

        return json.loads(map_path.read_text()), PIL.Image.open(map_image_path)
    else:
        left, right = load_base_images(Path(TEST_IMAGES_PATH) / name)
        composed = compose_base_images(left, right)
        base_image = reverse_projection(composed)

        return None, base_image


def main():
    map, base_image = get_map(argv[1])

    window = MainWindow(base_image)

    if map is not None:
        window.set_map(map)

    window.run()

    map_dir_path = Path(TEST_MAPS_PATH) / argv[1]

    map_dir_path.mkdir(parents=True, exist_ok=True)

    (map_dir_path / "map.json").write_text(
        json.dumps(window.get_map(), indent=4) + "\n"
    )
    if window.get_cropped_image_changed():
        window.cropped_image.save(map_dir_path / "map.jpg")


main()
