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
