import json
from io import BytesIO
from pathlib import Path
from zipfile import ZipFile

import PIL.Image

from .consts import *
from .map_model import MapModel, create_map_model


def load_test_map_raw(name: str) -> tuple[str, bytes]:
    zip_path = (Path(TEST_MAPS_PATH) / name).with_suffix(".zip")

    with ZipFile(zip_path) as zip_file:
        with zip_file.open("map.jpg") as map_image_file:
            map_image = map_image_file.read()
        with zip_file.open("map.json") as map_json_file:
            map = map_json_file.read()

    return map, map_image


def load_test_map(name: str) -> tuple[MapModel, PIL.Image.Image]:
    map, map_image = load_test_map_raw(name)

    map = create_map_model()(**json.loads(map))
    map_image = PIL.Image.open(BytesIO(map_image))

    return map, map_image


def round_floats(obj: object, ndigits: int | None) -> object:
    """Round floats in dict to ndigits."""

    if isinstance(obj, float):
        return round(obj, ndigits)
    if isinstance(obj, dict):
        return {key: round_floats(value, ndigits) for key, value in obj.items()}
    if isinstance(obj, list):
        return [round_floats(item, ndigits) for item in obj]
    if isinstance(obj, tuple):
        return tuple(round_floats(item, ndigits) for item in obj)

    return obj


def get_tile_color(
    even: bool, border: bool, drop_zone: bool, occupied: bool
) -> str:
    if occupied:
        return BUILDING_TILE_EVEN_COLOR if even else BUILDING_TILE_ODD_COLOR
    if drop_zone:
        if border:
            return (
                DROP_ZONE_BORDER_TILE_EVEN_COLOR
                if even
                else DROP_ZONE_BORDER_TILE_ODD_COLOR
            )
        else:
            return (
                DROP_ZONE_TILE_EVEN_COLOR if even else DROP_ZONE_TILE_ODD_COLOR
            )
    else:
        if border:
            return BORDER_TILE_EVEN_COLOR if even else BORDER_TILE_ODD_COLOR
        else:
            return TILE_EVEN_COLOR if even else TILE_ODD_COLOR


def distance(x1: float, y1: float, x2: float, y2: float) -> float:
    """Returns distance between points (x1, y1) and (x2, y2)."""

    return ((x1 - x2) ** 2 + (y1 - y2) ** 2) ** 0.5


def normalize(x: float, y: float) -> tuple[float, float]:
    """Normalize vector."""

    length = (x**2 + y**2) ** 0.5

    return x / length, y / length


def clamp(min_, value, max_):
    return max(min_, min(value, max_))


def check_intersection(
    x1: int, y1: int, x2: int, y2: int, field: list[list[object | None]]
) -> bool:
    """Check if line intersects at least one True value in field."""

    x_min, x_max = sorted((x1, x2))
    y_min, y_max = sorted((y1, y2))

    if x1 == x2:
        for y in range(y_min, y_max + 1):
            if field[x1][y] is not None:
                return True

        return False
    if y1 == y2:
        for x in range(x_min, x_max + 1):
            if field[x][y1] is not None:
                return True

        return False

    k = (y1 - y2) / (x1 - x2)
    b = (x1 * y2 - y1 * x2) / (x1 - x2)

    for x in range(x_min, x_max + 1):
        y = clamp(y_min, k * x + b, y_max)

        # ignore case when y is almost integer so (x, y) is almost at four tiles intersection

        if field[x][int(y)] is not None:
            return True

    for y in range(y_min, y_max + 1):
        x = clamp(x_min, (y - b) / k, x_max)

        # ignore case when x is almost integer so (x, y) is almost at four tiles intersection

        if field[int(x)][y] is not None:
            return True

    return False
