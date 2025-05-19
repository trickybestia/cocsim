import pygame

from .consts import *
from .utils import is_border
from .tile_color import get_tile_color


class Game:
    buildings: list[list[bool]]
    drop_zone: list[list[bool]]
    screen: pygame.Surface

    def __init__(self): ...

    def tick(self, delta_t: float): ...

    def draw(self):
        self._draw_grid()

    def _draw_grid(self):
        for tile_x in range(MAP_WIDTH):
            for tile_y in range(MAP_HEIGHT):
                pygame.draw.rect(
                    self.screen,
                    get_tile_color(
                        (tile_y ^ tile_x) & 1,
                        is_border(tile_x, tile_y),
                        self.drop_zone[tile_x][tile_y],
                    ),
                    (
                        tile_x * PIXELS_PER_CELL,
                        tile_y * PIXELS_PER_CELL,
                        PIXELS_PER_CELL,
                        PIXELS_PER_CELL,
                    ),
                )
