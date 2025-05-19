import pygame

from .consts import *
from .utils import is_border
from .tile_color import get_tile_color
from .buildings import Building
from .units import Unit


class Game:
    buildings: list[Building]
    occupied_tiles: list[list[bool]]
    drop_zone: list[list[bool]]
    units: list[Unit]

    screen: pygame.Surface

    def __init__(self): ...

    def tick(self, delta_t: float): ...

    def draw(self):
        self._draw_grid()

    def _draw_grid(self):
        for x in range(MAP_WIDTH):
            for y in range(MAP_HEIGHT):
                pygame.draw.rect(
                    self.screen,
                    get_tile_color(
                        (y ^ x) & 1,
                        is_border(x, y),
                        self.drop_zone[x][y],
                        self.occupied_tiles[x][y],
                    ),
                    (
                        x * PIXELS_PER_CELL,
                        y * PIXELS_PER_CELL,
                        PIXELS_PER_CELL,
                        PIXELS_PER_CELL,
                    ),
                )

    def compute_occupied_tiles(self):
        self.occupied_tiles = [[False] * MAP_HEIGHT for _ in range(MAP_WIDTH)]

        for building in self.buildings:
            for x, y in building.get_occupied_tiles():
                self.occupied_tiles[x][y] = True

    def compute_drop_zone(self):
        def get_neighbors(x: int, y: int) -> list[tuple[int, int]]:
            result = []

            for neighbor_x in range(x - 1, x + 2):
                for neighbor_y in range(y - 1, y + 2):
                    if (
                        0 <= neighbor_x < MAP_WIDTH
                        and 0 <= neighbor_y < MAP_HEIGHT
                    ):
                        result.append((neighbor_x, neighbor_y))

            return result

        self.drop_zone = [[True] * MAP_HEIGHT for _ in range(MAP_WIDTH)]

        for x in range(MAP_WIDTH):
            for y in range(MAP_HEIGHT):
                if self.occupied_tiles[x][y]:
                    for neighbor_x, neighbor_y in get_neighbors(x, y):
                        self.drop_zone[neighbor_x][neighbor_y] = False
