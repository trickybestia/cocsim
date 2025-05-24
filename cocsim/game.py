import pygame

from .consts import *
from .utils import is_border, get_tile_color
from . import buildings, units


class Game:
    buildings: list["buildings.Building"]
    occupied_tiles: list[list[bool]]
    drop_zone: list[list[bool]]
    collision: list[list[bool]]
    units: list["units.Unit"]

    time_left: float

    screen: pygame.Surface
    font: pygame.font.Font

    _townhall_destroyed: bool
    _destroyed_buildings_count: int

    @property
    def done(self) -> bool:
        return self.time_left == 0.0 or self.stars == 3

    @property
    def stars(self) -> int:
        return (
            int(self._townhall_destroyed)
            + int(self._destroyed_buildings_count / len(self.buildings) >= 0.5)
            + int(self._destroyed_buildings_count == len(self.buildings))
        )

    def __init__(self):
        self.time_left = 180.0
        self.font = pygame.font.SysFont("Arial", 30)
        self._townhall_destroyed = False
        self._destroyed_buildings_count = 0

    def tick(self, delta_t: float):
        assert not self.done

        for building in self.buildings:
            building.tick(delta_t)

        for unit in self.units:
            unit.tick(delta_t)

        self.time_left = max(0.0, self.time_left - delta_t)

    def draw(self):
        self._draw_grid()
        self._draw_collision()

        for building in self.buildings:
            building.draw()

        for unit in self.units:
            unit.draw()

        self._draw_timer()

    def building_destroyed(self, building: "buildings.Building"):
        """Called once by every Building when it gets destroyed."""

        if isinstance(building, buildings.TownHall):
            self._townhall_destroyed = True

        self._destroyed_buildings_count += 1

    def _draw_grid(self):
        for x in range(MAP_SIZE):
            for y in range(MAP_SIZE):
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

        for x in range(MAP_SIZE * COLLISION_TILES_PER_MAP_TILE):
            for y in range(MAP_SIZE * COLLISION_TILES_PER_MAP_TILE):
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

    def _draw_timer(self):
        seconds = int(self.time_left)
        minutes = seconds // 60
        seconds %= 60

        text = f"{int(self._destroyed_buildings_count / len(self.buildings) * 100.0)} % | {self.stars} star |"

        if minutes != 0:
            text += f" {minutes} min"

        text += f" {seconds} s left"

        text_surface = self.font.render(text, True, TIMER_COLOR)

        self.screen.blit(text_surface, TIMER_POSITION)

    def compute_collision(self):
        self.collision = [
            [False] * MAP_SIZE * COLLISION_TILES_PER_MAP_TILE
            for _ in range(MAP_SIZE * COLLISION_TILES_PER_MAP_TILE)
        ]

        for building in self.buildings:
            building.update_collision()

    def compute_occupied_tiles(self):
        self.occupied_tiles = [[False] * MAP_SIZE for _ in range(MAP_SIZE)]

        for building in self.buildings:
            building.occupy_tiles()

    def compute_drop_zone(self):
        def get_neighbors(x: int, y: int) -> list[tuple[int, int]]:
            result = []

            for neighbor_x in range(x - 1, x + 2):
                for neighbor_y in range(y - 1, y + 2):
                    if (
                        0 <= neighbor_x < MAP_SIZE
                        and 0 <= neighbor_y < MAP_SIZE
                    ):
                        result.append((neighbor_x, neighbor_y))

            return result

        self.drop_zone = [[True] * MAP_SIZE for _ in range(MAP_SIZE)]

        for x in range(MAP_SIZE):
            for y in range(MAP_SIZE):
                if self.occupied_tiles[x][y]:
                    for neighbor_x, neighbor_y in get_neighbors(x, y):
                        self.drop_zone[neighbor_x][neighbor_y] = False
