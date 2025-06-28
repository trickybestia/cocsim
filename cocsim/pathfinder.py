import heapq
from itertools import takewhile
from math import inf
from typing import Callable, Type

from . import buildings, game, units
from .consts import *
from .utils import check_intersection, distance


class Pathfinder:
    game: "game.Game"

    def __init__(self, game: "game.Game"):
        self.game = game

    def get_targets(
        self,
        unit: "units.Unit",
        preferred_building: Type["buildings.Building"] | None,
    ) -> list["buildings.Building"]:
        if preferred_building is not None:
            priority_func = lambda b: (
                isinstance(b, preferred_building),
                not isinstance(b, buildings.Wall),
            )
        else:
            priority_func = lambda b: not isinstance(b, buildings.Wall)

        buildings_: list[tuple[object, buildings.Building]] = sorted(
            (
                (priority_func(b), b)
                for b in self.game.buildings
                if not b.destroyed and b.collider is not None
            ),
            key=lambda t: t[0],
            reverse=True,
        )

        buildings_ = list(
            takewhile(lambda t: t[0] == buildings_[0][0], buildings_)
        )

        targets: list[tuple[float, buildings.Building]] = []

        for _, building in buildings_:
            nearest_point = building.collider.get_attack_area(
                unit.attack_range
            ).get_nearest_point(unit.x, unit.y)
            targets.append(
                (
                    distance(
                        nearest_point[0], nearest_point[1], unit.x, unit.y
                    ),
                    building,
                )
            )

        targets.sort(key=lambda t: t[0])

        return [t[1] for t in targets]

    def find_best_air_path(
        self,
        unit: "units.Unit",
        preferred_building: Type["buildings.Building"] | None,
    ) -> tuple["buildings.Building", list[tuple[float, float]]]:
        targets = self.get_targets(unit, preferred_building)

        return targets[0], self.find_air_path(unit, targets[0])

    def find_air_path(
        self, unit: "units.Unit", target: "buildings.Building"
    ) -> list[tuple[float, float]]:
        nearest_point = target.collider.get_attack_area(
            unit.attack_range
        ).get_nearest_point(unit.x, unit.y)

        return [nearest_point]

    def find_best_ground_path(
        self,
        unit: "units.Unit",
        preferred_building: Type["buildings.Building"] | None,
    ) -> tuple["buildings.Building", list[tuple[float, float]]]:
        targets = self.get_targets(unit, preferred_building)[:2]

        paths = [self.find_ground_path(unit, target) for target in targets]
        best_path = min(paths, key=lambda t: t[0])

        return best_path[2]()

    def find_ground_path(
        self, unit: "units.Unit", target: "buildings.Building"
    ) -> tuple[
        float,
        "buildings.Building",
        Callable[[], tuple["buildings.Building", list[tuple[float, float]]]],
    ]:
        nearest_point = target.collider.get_attack_area(
            unit.attack_range
        ).get_nearest_point(unit.x, unit.y)
        nearest_point_x = int(nearest_point[0] * COLLISION_TILES_PER_MAP_TILE)
        nearest_point_y = int(nearest_point[1] * COLLISION_TILES_PER_MAP_TILE)

        def get_tile_to_check_priority(x: int, y: int) -> int:
            return (
                abs(x - nearest_point_x)
                + abs(y - nearest_point_y)
                + self._get_tile_penalty(x, y)
            )

        distances = [
            [inf] * self.game.total_size * COLLISION_TILES_PER_MAP_TILE
            for _ in range(self.game.total_size * COLLISION_TILES_PER_MAP_TILE)
        ]
        checked_tiles = [
            [False] * self.game.total_size * COLLISION_TILES_PER_MAP_TILE
            for _ in range(self.game.total_size * COLLISION_TILES_PER_MAP_TILE)
        ]

        start_x = int(unit.x * COLLISION_TILES_PER_MAP_TILE)
        start_y = int(unit.y * COLLISION_TILES_PER_MAP_TILE)

        tiles_to_check = []  # heapq's heap
        heapq.heappush(tiles_to_check, (0, start_x, start_y))

        distances[start_x][start_y] = 0
        checked_tiles[start_x][start_y] = True

        while (
            len(tiles_to_check) != 0
            and distances[nearest_point_x][nearest_point_y] == inf
        ):
            _, x, y = heapq.heappop(tiles_to_check)

            for neighbor_x, neighbor_y in self._get_neighbors(x, y):
                distances[neighbor_x][neighbor_y] = min(
                    distances[neighbor_x][neighbor_y],
                    distances[x][y]
                    + self._get_tile_penalty(neighbor_x, neighbor_y),
                )

                if not checked_tiles[neighbor_x][neighbor_y]:
                    heapq.heappush(
                        tiles_to_check,
                        (
                            get_tile_to_check_priority(neighbor_x, neighbor_y),
                            neighbor_x,
                            neighbor_y,
                        ),
                    )

                    checked_tiles[neighbor_x][neighbor_y] = True

        def get_path():
            nonlocal target

            collision_waypoints = [(nearest_point_x, nearest_point_y)]

            while collision_waypoints[-1] != (start_x, start_y):
                x, y = collision_waypoints[-1]

                min_distance_neighbor = None
                min_distance_neighbor_distance = None

                for neighbor_x, neighbor_y in self._get_neighbors(x, y):
                    neighbor_distance = distances[neighbor_x][neighbor_y]

                    if (
                        min_distance_neighbor_distance is None
                        or neighbor_distance < min_distance_neighbor_distance
                    ):
                        min_distance_neighbor = (neighbor_x, neighbor_y)
                        min_distance_neighbor_distance = neighbor_distance

                collision_waypoints.append(min_distance_neighbor)

            collision_waypoints = collision_waypoints[::-1]

            for i in range(len(collision_waypoints)):
                waypoint_x, waypoint_y = collision_waypoints[i]

                building = self.game.collision_grid[waypoint_x][waypoint_y]

                if building is not None:
                    target = building
                    collision_waypoints = collision_waypoints[:i]

                    break

            if len(collision_waypoints) == 0:
                collision_waypoints = [
                    (
                        int(unit.x * COLLISION_TILES_PER_MAP_TILE),
                        int(unit.y * COLLISION_TILES_PER_MAP_TILE),
                    )
                ]

            collision_waypoints = self.game.pathfinder._simplify_path(
                collision_waypoints
            )

            return target, [
                (
                    x / COLLISION_TILES_PER_MAP_TILE,
                    y / COLLISION_TILES_PER_MAP_TILE,
                )
                for x, y in collision_waypoints
            ]

        return distances[nearest_point_x][nearest_point_y], target, get_path

    def _get_tile_penalty(self, x: int, y: int) -> float:
        building = self.game.collision_grid[x][y]

        if building is None:
            return 1.0
        if isinstance(building, buildings.Wall):
            return 200.0

        return inf

    def _get_neighbors(self, x: int, y: int) -> list[tuple[int, int]]:
        """Returns list of neighbor tiles on Game.collision_grid."""

        result = []

        for neighbor_x, neighbor_y in (
            (x, y - 1),
            (x + 1, y),
            (x, y + 1),
            (x - 1, y),
        ):
            if (
                0
                <= neighbor_x
                < self.game.total_size * COLLISION_TILES_PER_MAP_TILE
                and 0
                <= neighbor_y
                < self.game.total_size * COLLISION_TILES_PER_MAP_TILE
            ):
                result.append((neighbor_x, neighbor_y))

        return result

    def _simplify_path(
        self, path: list[tuple[int, int]]
    ) -> list[tuple[int, int]]:
        """Linearize Γ-like motion where possible allowing diagonal movement."""

        result = [path[0]]

        for i in range(1, len(path)):
            waypoint = path[i]

            if check_intersection(
                result[-1][0],
                result[-1][1],
                waypoint[0],
                waypoint[1],
                self.game.collision_grid,
            ):
                result.append(path[i - 1])

        if result[-1] != path[-1]:
            result.append(path[-1])

        return result
