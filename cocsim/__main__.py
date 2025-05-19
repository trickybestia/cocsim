import pygame

from .tile_color import get_tile_color

BASE_WIDTH = 44
BASE_HEIGHT = 44
MAP_BORDERS = 3

MAP_WIDTH = BASE_WIDTH + MAP_BORDERS * 2
MAP_HEIGHT = BASE_HEIGHT + MAP_BORDERS * 2

PIXELS_PER_CELL = 20


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


def draw_grid(screen: pygame.Surface, drop_zone: list[list[bool]]):
    for tile_x in range(MAP_WIDTH):
        for tile_y in range(MAP_HEIGHT):
            is_border = (
                tile_y < MAP_BORDERS
                or tile_x < MAP_BORDERS
                or tile_y >= BASE_HEIGHT + MAP_BORDERS
                or tile_x >= BASE_WIDTH + MAP_BORDERS
            )

            pygame.draw.rect(
                screen,
                get_tile_color(
                    (tile_y ^ tile_x) & 1,
                    is_border,
                    drop_zone[tile_x][tile_y],
                ),
                (
                    tile_x * PIXELS_PER_CELL,
                    tile_y * PIXELS_PER_CELL,
                    PIXELS_PER_CELL,
                    PIXELS_PER_CELL,
                ),
            )


def main():
    pygame.init()

    screen = pygame.display.set_mode(
        (MAP_WIDTH * PIXELS_PER_CELL, MAP_HEIGHT * PIXELS_PER_CELL)
    )

    buildings = [[False] * MAP_HEIGHT for _ in range(MAP_WIDTH)]
    buildings[10][10] = True

    drop_zone = compute_drop_zone(buildings)

    draw_grid(screen, drop_zone)

    pygame.display.update()

    input()


main()
