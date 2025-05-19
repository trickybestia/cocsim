from .consts import *


def is_border(tile_x: int, tile_y: int) -> bool:
    return (
        tile_y < MAP_BORDERS
        or tile_x < MAP_BORDERS
        or tile_y >= BASE_HEIGHT + MAP_BORDERS
        or tile_x >= BASE_WIDTH + MAP_BORDERS
    )


def compute_drop_zone(buildings: list[list[bool]]) -> list[list[bool]]:
    def get_neighbors(tile_x: int, tile_y: int) -> list[tuple[int, int]]:
        result = []

        for neighbor_x in range(tile_x - 1, tile_x + 2):
            for neighbor_y in range(tile_y - 1, tile_y + 2):
                if 0 <= neighbor_x < MAP_WIDTH and 0 <= neighbor_y < MAP_HEIGHT:
                    result.append((neighbor_x, neighbor_y))

        return result

    result = [[True] * MAP_HEIGHT for _ in range(MAP_WIDTH)]

    for tile_x in range(MAP_WIDTH):
        for tile_y in range(MAP_HEIGHT):
            if buildings[tile_x][tile_y]:
                for neighbor_x, neighbor_y in get_neighbors(tile_x, tile_y):
                    result[neighbor_x][neighbor_y] = False

    return result
