import pygame

from .consts import *
from .utils import is_border, get_tile_color
from .buildings import Building
from . import units


class Game:
    buildings: list[Building]
    occupied_tiles: list[list[bool]]
    drop_zone: list[list[bool]]
    collision: list[list[bool]]
    units: list["units.Unit"]

    screen: pygame.Surface

    def __init__(self): ...

    def tick(self, delta_t: float):
        for building in self.buildings:
            building.tick(delta_t)

        for unit in self.units:
            unit.tick(delta_t)

    def draw(self):
        self._draw_grid()
        self._draw_collision()

        for building in self.buildings:
            building.draw()

        for unit in self.units:
            unit.draw()

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
                        x * PIXELS_PER_TILE,
                        y * PIXELS_PER_TILE,
                        PIXELS_PER_TILE,
                        PIXELS_PER_TILE,
                    ),
                )

    def _draw_collision(self):
        PIXELS_PER_COLLISION_TILE = (
            PIXELS_PER_TILE / COLLISION_TILES_PER_MAP_TILE
        )

        collision_surface = pygame.Surface(self.screen.get_size())

        collision_surface.set_alpha(100)

        for x in range(MAP_WIDTH * COLLISION_TILES_PER_MAP_TILE):
            for y in range(MAP_HEIGHT * COLLISION_TILES_PER_MAP_TILE):
                if self.collision[x][y]:
                    pygame.draw.rect(
                        collision_surface,
                        COLLISION_TILE_COLOR,
                        (
                            x * PIXELS_PER_COLLISION_TILE,
                            y * PIXELS_PER_COLLISION_TILE,
                            PIXELS_PER_COLLISION_TILE,
                            PIXELS_PER_COLLISION_TILE,
                        ),
                    )

        self.screen.blit(collision_surface, (0, 0))

    def compute_collision(self):
        self.collision = [
            [False] * MAP_HEIGHT * COLLISION_TILES_PER_MAP_TILE
            for _ in range(MAP_WIDTH * COLLISION_TILES_PER_MAP_TILE)
        ]

        for building in self.buildings:
            building.update_collision()

    def compute_occupied_tiles(self):
        self.occupied_tiles = [[False] * MAP_HEIGHT for _ in range(MAP_WIDTH)]

        for building in self.buildings:
            building.occupy_tiles()

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
