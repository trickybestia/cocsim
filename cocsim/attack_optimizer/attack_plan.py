from dataclasses import dataclass
from math import pi
from random import choice, random
from typing import Type

from cocsim.consts import *
from cocsim.game import Game
from cocsim.units import Unit
from cocsim.utils import clamp

from .geometry import Point, Ray, Segment, Square

MAX_UNIT_DROP_TIME = 20


@dataclass
class AttackPlanUnit:
    unit: Type[Unit]
    level: int
    angle: float  # radians
    distance: float  # from 0 to 1
    drop_time: float

    def cartesian_pos(self, game: Game) -> tuple[float, float]:
        offset = (game.base_size + game.border_size) / 2
        center = Point(offset, offset)

        ray = Ray(center, self.angle)
        border_square = Square(center, game.base_size + game.border_size)

        start_point = None
        stop_point = ray.intersection_with_square(border_square)
        segment_length = (stop_point - ray.start).length()

        for t in range(100):
            distance = segment_length * t / 100
            tile_pos = stop_point - ray.direction * distance
            x = clamp(0, int(tile_pos.x), game.total_size - 1)
            y = clamp(0, int(tile_pos.y), game.total_size - 1)

            if (
                game.is_inside_map(x + 1, y)
                and game.drop_zone[x][y] != game.drop_zone[x + 1][y]
            ):  # right tile border
                start_point = ray.intersection_with_segment(
                    Segment(Point(x + 1, y), Point(x + 1, y + 1))
                )

                if start_point is not None:
                    break

            if (
                game.is_inside_map(x, y + 1)
                and game.drop_zone[x][y] != game.drop_zone[x][y + 1]
            ):  # down tile border
                start_point = ray.intersection_with_segment(
                    Segment(Point(x, y + 1), Point(x + 1, y + 1))
                )

                if start_point is not None:
                    break

            if (
                game.is_inside_map(x - 1, y)
                and game.drop_zone[x][y] != game.drop_zone[x - 1][y]
            ):  # left tile border
                start_point = ray.intersection_with_segment(
                    Segment(Point(x, y), Point(x, y + 1))
                )

                if start_point is not None:
                    break

            if (
                game.is_inside_map(x, y - 1)
                and game.drop_zone[x][y] != game.drop_zone[x][y - 1]
            ):  # up tile border
                start_point = ray.intersection_with_segment(
                    Segment(Point(x, y), Point(x + 1, y))
                )

                if start_point is not None:
                    break

        assert start_point is not None

        result = start_point + (stop_point - start_point) * min(
            0.99, self.distance
        )  # min for unit to not spawn on right or bottom border

        return result.x, result.y


class AttackPlan:
    units: list[AttackPlanUnit]

    @classmethod
    def randomize(cls, units: list[tuple[Type[Unit], int]]) -> "AttackPlan":
        result = AttackPlan([])

        for unit in units:
            angle = random() * 2 * pi
            distance = random()
            drop_time = random() * MAX_UNIT_DROP_TIME

            result.units.append(
                AttackPlanUnit(unit[0], unit[1], angle, distance, drop_time)
            )

        return result

    @classmethod
    def merge(cls, a: "AttackPlan", b: "AttackPlan") -> "AttackPlan":
        result = AttackPlan([])

        for i in range(len(a.units)):
            result.units.append(choice((a.units[i], b.units[i])))

        return result

    def __init__(self, units: list[AttackPlanUnit]):
        self.units = units

    def mutate(self) -> "AttackPlan":
        result = AttackPlan([])

        for unit in self.units:
            angle = unit.angle * (1 + (random() - 0.5) * 0.2)
            distance = clamp(
                0.0, unit.distance * (1 + (random() - 0.5) * 0.2), 1.0
            )
            drop_time = unit.drop_time * (1 + (random() - 0.8) * 0.2)

            result.units.append(
                AttackPlanUnit(
                    unit.unit, unit.level, angle, distance, drop_time
                )
            )

        return result
