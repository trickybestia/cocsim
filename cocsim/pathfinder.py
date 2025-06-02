from typing import Type
from itertools import takewhile

from .utils import check_intersection, distance
from .buildings import Building, Wall
from . import game, units


class Pathfinder:
    game: "game.Game"

    def __init__(self, game: "game.Game"):
        self.game = game

    def find_target(
        self, unit: "units.Unit", preferred_building: Type[Building] | None
    ) -> Building | None:
        if preferred_building is not None:
            priority_func = lambda b: isinstance(b, preferred_building)
        else:
            priority_func = lambda b: not isinstance(b, Wall)

        buildings: list[tuple[object, Building]] = sorted(
            (
                (priority_func(b), b)
                for b in self.game.buildings
                if not b.destroyed and b.collider is not None
            ),
            key=lambda t: t[0],
            reverse=True,
        )

        buildings = list(
            takewhile(lambda t: t[0] == buildings[0][0], buildings)
        )

        if len(buildings) == 0:
            return

        targets: list[tuple[float, Building]] = []

        for _, building in buildings:
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

        return min(targets, key=lambda t: t[0])[1]

    def find_path(
        self, unit: "units.Unit", target: Building
    ) -> tuple[Building, list[tuple[float, float]]]: ...

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
