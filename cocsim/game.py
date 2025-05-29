import pygame

from .consts import *
from .utils import get_tile_color
from .map import Map
from . import buildings, units


class Game:
    base_size: int
    border_size: int

    buildings: list["buildings.Building"]
    occupied_tiles: list[list[bool]]
    drop_zone: list[list[bool]]
    collision: list[list[bool]]
    units: list["units.Unit"]

    time_left: float

    screen: pygame.Surface

    _townhall_destroyed: bool
    _destroyed_buildings_count: int
    _total_buildings_count: int

    @property
    def total_size(self) -> int:
        return self.base_size + 2 * self.border_size

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

    def __init__(self, map: Map):
        self.base_size = map["base_size"]
        self.border_size = map["border_size"]

        self.buildings = []
        self.time_left = 180.0
        self._townhall_destroyed = False
        self._destroyed_buildings_count = 0
        self._total_buildings_count = 0

        for building in map["buildings"]:
            self.buildings.append(
                buildings.BUILDINGS_DICT[building["name"]](
                    self, building["x"], building["y"], building["level"]
                )
            )

        self.compute_buildings_count()
        self.compute_collision()
        self.compute_occupied_tiles()
        self.compute_drop_zone()

    def is_border(self, x: int, y: int) -> bool:
        return (
            y < self.border_size
            or x < self.border_size
            or y >= self.base_size + self.border_size
            or x >= self.base_size + self.border_size
        )

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

        self.progress_info()

    def building_destroyed(self, building: "buildings.Building"):
        """Called once by every Building when it gets destroyed."""

        if isinstance(building, buildings.TownHall):
            self._townhall_destroyed = True

        self._destroyed_buildings_count += 1

    def progress_info(self):
        seconds = int(self.time_left)
        minutes = seconds // 60
        seconds %= 60

        text = f"{int(self._destroyed_buildings_count / len(self.buildings) * 100.0)} % | {self.stars} star |"

        if minutes != 0:
            text += f" {minutes} min"

        text += f" {seconds} s left"

        return text

    def _draw_grid(self):
        for x in range(self.total_size):
            for y in range(self.total_size):
                pygame.draw.rect(
                    self.screen,
                    get_tile_color(
                        (y ^ x) & 1,
                        self.is_border(x, y),
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

        for x in range(self.total_size * COLLISION_TILES_PER_MAP_TILE):
            for y in range(self.total_size * COLLISION_TILES_PER_MAP_TILE):
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

    def compute_buildings_count(self):
        self._total_buildings_count = 0

        for building in self.buildings:
            if not isinstance(building, buildings.Wall):
                self._total_buildings_count += 1

    def compute_collision(self):
        self.collision = [
            [False] * self.total_size * COLLISION_TILES_PER_MAP_TILE
            for _ in range(self.total_size * COLLISION_TILES_PER_MAP_TILE)
        ]

        for building in self.buildings:
            building.update_collision()

    def compute_occupied_tiles(self):
        self.occupied_tiles = [
            [False] * self.total_size for _ in range(self.total_size)
        ]

        for building in self.buildings:
            building.occupy_tiles()

    def compute_drop_zone(self):
        def get_neighbors(x: int, y: int) -> list[tuple[int, int]]:
            result = []

            for neighbor_x in range(x - 1, x + 2):
                for neighbor_y in range(y - 1, y + 2):
                    if (
                        0 <= neighbor_x < self.total_size
                        and 0 <= neighbor_y < self.total_size
                    ):
                        result.append((neighbor_x, neighbor_y))

            return result

        self.drop_zone = [
            [True] * self.total_size for _ in range(self.total_size)
        ]

        for x in range(self.total_size):
            for y in range(self.total_size):
                if self.occupied_tiles[x][y]:
                    for neighbor_x, neighbor_y in get_neighbors(x, y):
                        self.drop_zone[neighbor_x][neighbor_y] = False
