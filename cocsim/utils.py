from .consts import *


def is_border(x: int, y: int) -> bool:
    return (
        y < MAP_BORDERS
        or x < MAP_BORDERS
        or y >= BASE_SIZE + MAP_BORDERS
        or x >= BASE_SIZE + MAP_BORDERS
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


def check_intersection(
    x1: int, y1: int, x2: int, y2: int, field: list[list[bool]]
) -> bool:
    """Check if line intersects at least one True value in field."""

    x_min, x_max = sorted((x1, x2))
    y_min, y_max = sorted((y1, y2))

    if x1 == x2:
        for y in range(y_min, y_max + 1):
            if field[x1][y]:
                return True

        return False
    if y1 == y2:
        for x in range(x_min, x_max + 1):
            if field[x][y1]:
                return True

        return False

    k = (y1 - y2) / (x1 - x2)
    b = (x1 * y2 - y1 * x2) / (x1 - x2)

    for x in range(x_min, x_max + 1):
        y = k * x + b

        # ignore case when y is almost integer so (x, y) is almost at four tiles intersection

        if field[x][int(y)]:
            return True

    for y in range(y_min, y_max + 1):
        x = (y - b) / k

        # ignore case when x is almost integer so (x, y) is almost at four tiles intersection

        if field[int(x)][y]:
            return True

    return False
