from .consts import *


def is_border(x: int, y: int) -> bool:
    return (
        y < MAP_BORDERS
        or x < MAP_BORDERS
        or y >= BASE_HEIGHT + MAP_BORDERS
        or x >= BASE_WIDTH + MAP_BORDERS
    )


def get_tile_color(
    even: bool, border: bool, drop_zone: bool, occupied: bool
) -> Color:
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
