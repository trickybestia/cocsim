from typing import Union

from . import buildings, units
from .consts import *
from .map_model import MapModel
from .pathfinder import Pathfinder
from .shapes import *
from .utils import draw_bool_grid, get_tile_color


class Game:
    base_size: int
    border_size: int

    buildings: list["buildings.Building"]
    units: list["units.Unit"]

    buildings_grid: list[list[Union["buildings.Building", None]]]
    drop_zone: list[list[bool]]
    collision_grid: list[list[Union["buildings.Building", None]]]

    pathfinder: Pathfinder

    time_elapsed: float

    _townhall_destroyed: bool
    _destroyed_buildings_count: int
    _total_buildings_count: int
    _need_redraw_collision: bool

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

    def __init__(self, map: MapModel):
        self.base_size = map.base_size
        self.border_size = map.border_size

        self.buildings = []
        self.units = []

        self.pathfinder = Pathfinder(self)

        self.time_elapsed = 0.0

        self._townhall_destroyed = False
        self._destroyed_buildings_count = 0
        self._total_buildings_count = 0
        self._need_redraw_collision = True

        for building_model in map.buildings:
            building_type = buildings.BUILDINGS_DICT[building_model.name]
            building: buildings.Building = building_type.from_model(
                self, building_model
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

    def progress_info(self):
        seconds = int(self.time_left)
        minutes = seconds // 60
        seconds %= 60

        text = f"{round(self._destroyed_buildings_count * 100.0 / self._total_buildings_count )} % | {self.stars} star |"

        if minutes != 0:
            text += f" {minutes} min"

        text += f" {seconds} s left"

        return text

    def draw_entities(self) -> list[Shape]:
        result = []

        for building in self.buildings:
            building.draw(result)

        for unit in self.units:
            unit.draw(result)

        return result

    def draw_grid(self) -> list[Shape]:
        result = []

        for x in range(self.total_size):
            for y in range(self.total_size):
                result.append(
                    rect(
                        x,
                        y,
                        1,
                        1,
                        get_tile_color(
                            (y ^ x) & 1,
                            self.is_border(x, y),
                            self.drop_zone[x][y],
                            self.buildings_grid[x][y] is not None,
                        ),
                    )
                )

        return result

    def need_redraw_collision(self) -> bool:
        return self._need_redraw_collision

    def draw_collision(self) -> list[Shape]:
        self._need_redraw_collision = False

        return draw_bool_grid(
            [
                [
                    self.collision_grid[x][y] is not None
                    for y in range(
                        self.total_size * COLLISION_TILES_PER_MAP_TILE
                    )
                ]
                for x in range(self.total_size * COLLISION_TILES_PER_MAP_TILE)
            ],
            COLLISION_TILE_SIZE,
            COLLISION_TILE_COLOR,
        )

    def _on_building_destroyed(self, building: "buildings.Building"):
        self._need_redraw_collision = True

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
