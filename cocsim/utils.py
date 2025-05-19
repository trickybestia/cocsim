from .consts import *


def is_border(x: int, y: int) -> bool:
    return (
        y < MAP_BORDERS
        or x < MAP_BORDERS
        or y >= BASE_HEIGHT + MAP_BORDERS
        or x >= BASE_WIDTH + MAP_BORDERS
    )
