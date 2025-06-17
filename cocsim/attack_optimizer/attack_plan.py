from typing import Type
from dataclasses import dataclass
from random import random
from math import pi

from cocsim.units import Unit
from cocsim.consts import *

from .geometry import Ray, Point, Square


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
            drop_time = random() * MAX_ATTACK_DURATION

            result.units.append(
                AttackPlanUnit(unit[0], unit[1], angle, distance, drop_time)
            )

        return result

    def __init__(self, units: list[AttackPlanUnit]):
        self.units = units
