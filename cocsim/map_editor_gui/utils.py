from typing import Type

from rapidfuzz import fuzz
from rapidfuzz.utils import default_process

from cocsim.consts import *
from cocsim.buildings import BUILDINGS, Building, Wall


def get_buildings_with_size(size: tuple[int, int]) -> list[Type[Building]]:
    result = [Wall]

    for building in BUILDINGS:
        if building is Wall:
            continue

        if building.width() == size[0] and building.height() == size[1]:
            result.append(building)

    return result


def buildings_fuzzy_sort(search_text: str, buildings: list[Type[Building]]):
    buildings.sort(
        reverse=True,
        key=lambda b: fuzz.ratio(
            search_text, b.__name__, processor=default_process
        ),
    )
