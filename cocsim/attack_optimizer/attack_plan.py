from typing import Type
from dataclasses import dataclass
from random import random, choice
from math import pi

from cocsim.units import Unit
from cocsim.consts import *

from .geometry import Ray, Point, Square

MAX_UNIT_DROP_TIME = 20


@dataclass
class AttackPlanUnit:
    unit: Type[Unit]
    level: int
    angle: float  # radians
    distance: float  # from 0 to 1
    drop_time: float

    def cartesian_pos(
        self, base_size: int, border_size: int
    ) -> tuple[float, float]:
        ray = Ray(Point(0.0, 0.0), self.angle)
        base_square = Square(Point(0.0, 0.0), base_size)
        border_square = Square(Point(0.0, 0.0), base_size + border_size)

        start_point = ray.intersection_with_square(base_square)
        stop_point = ray.intersection_with_square(border_square)

        result = start_point + (stop_point - start_point) * self.distance
        result -= (
            result * 0.01
        )  # for unit to not spawn on right or bottom border

        offset = (base_size + border_size) / 2

        return result.x + offset, result.y + offset


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
            distance = max(
                0.0, min(unit.distance * (1 + (random() - 0.5) * 0.2), 1.0)
            )
            drop_time = unit.drop_time * (1 + (random() - 0.8) * 0.2)

            result.units.append(
                AttackPlanUnit(
                    unit.unit, unit.level, angle, distance, drop_time
                )
            )

        return result
