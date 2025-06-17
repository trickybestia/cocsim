from typing import Union

import pygame
import PIL.Image

from .consts import *
from .utils import get_tile_color
from .map import Map
from .pathfinder import Pathfinder
from . import buildings, units


class Game:
    base_size: int
    border_size: int

    buildings: list["buildings.Building"]
    buildings_grid: list[list[Union["buildings.Building", None]]]
    drop_zone: list[list[bool]]
    collision_grid: list[list[Union["buildings.Building", None]]]
    units: list["units.Unit"]

    pathfinder: Pathfinder

    time_elapsed: float

    screen: pygame.Surface
    _base_image: pygame.Surface

    _townhall_destroyed: bool
    _destroyed_buildings_count: int
    _total_buildings_count: int

    @property
    def time_left(self) -> float:
        return MAX_ATTACK_DURATION - self.time_elapsed

    @property
    def total_size(self) -> int:
        return self.base_size + 2 * self.border_size

    @property
    def done(self) -> bool:
        return self.time_elapsed == MAX_ATTACK_DURATION or self.stars == 3

    @property
    def stars(self) -> int:
        return (
            int(self._townhall_destroyed)
            + int(
                round(
                    self._destroyed_buildings_count
                    * 100.0
                    / self._total_buildings_count
                )
                >= 50
            )
            + int(
                self._destroyed_buildings_count == self._total_buildings_count
            )
        )

    def __init__(self, map: Map, base_image: PIL.Image.Image | None):
        self.base_size = map["base_size"]
        self.border_size = map["border_size"]

        self.buildings = []
        self.time_elapsed = 0.0
        self._townhall_destroyed = False
        self._destroyed_buildings_count = 0
        self._total_buildings_count = 0

        self.pathfinder = Pathfinder(self)

        if base_image is not None:
            base_image = base_image.resize(
                (
                    self.total_size * PIXELS_PER_TILE,
                    self.total_size * PIXELS_PER_TILE,
                )
            )
            self._base_image = pygame.image.frombytes(
                base_image.tobytes(), base_image.size, base_image.mode
            )
            self._base_image.set_alpha(100)
        else:
            self._base_image = None

        for building_dto in map["buildings"]:
            building = buildings.BUILDINGS_DICT[building_dto["name"]](
                self,
                building_dto["x"],
                building_dto["y"],
                building_dto["level"],
            )
            building.on_destroyed.add(self._on_building_destroyed)

            self.buildings.append(building)

        self.compute_buildings_count()
        self.compute_occupied_tiles()
        self.compute_drop_zone()
        self.compute_collision()

    def is_border(self, x: int, y: int) -> bool:
        return (
            y < self.border_size
            or x < self.border_size
            or y >= self.base_size + self.border_size
            or x >= self.base_size + self.border_size
        )

    def is_inside_map(self, x: int, y: int) -> bool:
        return 0 <= x < self.total_size and 0 <= y < self.total_size

    def tick(self, delta_t: float):
        assert not self.done

        for building in self.buildings:
            building.tick(delta_t)

        for unit in self.units:
            unit.tick(delta_t)

        self.time_elapsed = min(
            MAX_ATTACK_DURATION, self.time_elapsed + delta_t
        )

    def draw(self):
        self._draw_grid()
        self._draw_collision()

        if self._base_image is not None:
            self.screen.blit(self._base_image, (0, 0))

        for building in self.buildings:
            building.draw()

        for unit in self.units:
            unit.draw()

    def progress_info(self):
        seconds = int(self.time_left)
        minutes = seconds // 60
        seconds %= 60

        text = f"{round(self._destroyed_buildings_count * 100.0 / self._total_buildings_count )} % | {self.stars} star |"

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
                        self.buildings_grid[x][y] is not None,
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

        collision_surface.set_alpha(150)

        for x in range(self.total_size * COLLISION_TILES_PER_MAP_TILE):
            for y in range(self.total_size * COLLISION_TILES_PER_MAP_TILE):
                if self.collision_grid[x][y] is not None:
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

    def _on_building_destroyed(self, building: "buildings.Building"):
        if isinstance(building, buildings.TownHall):
            self._townhall_destroyed = True

        if not isinstance(building, buildings.Wall):
            self._destroyed_buildings_count += 1

    def compute_buildings_count(self):
        self._total_buildings_count = 0

        for building in self.buildings:
            if not isinstance(building, buildings.Wall):
                self._total_buildings_count += 1

    def compute_collision(self):
        self.collision_grid = [
            [None] * self.total_size * COLLISION_TILES_PER_MAP_TILE
            for _ in range(self.total_size * COLLISION_TILES_PER_MAP_TILE)
        ]

        for building in self.buildings:
            building.update_collision()

    def compute_occupied_tiles(self):
        self.buildings_grid = [
            [None] * self.total_size for _ in range(self.total_size)
        ]

        for building in self.buildings:
            building.occupy_tiles()

    def compute_drop_zone(self):
        def get_neighbors(x: int, y: int) -> list[tuple[int, int]]:
            result = []

            for neighbor_x in range(x - 1, x + 2):
                for neighbor_y in range(y - 1, y + 2):
                    if self.is_inside_map(neighbor_x, neighbor_y):
                        result.append((neighbor_x, neighbor_y))

            return result

        self.drop_zone = [
            [True] * self.total_size for _ in range(self.total_size)
        ]

        for x in range(self.total_size):
            for y in range(self.total_size):
                if self.buildings_grid[x][y] is not None:
                    for neighbor_x, neighbor_y in get_neighbors(x, y):
                        self.drop_zone[neighbor_x][neighbor_y] = False
