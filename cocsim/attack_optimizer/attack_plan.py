from typing import Type
from dataclasses import dataclass
from random import random
from math import pi

import sympy

from cocsim.units import Unit
from cocsim.consts import *


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
        ray = sympy.Ray((0, 0), angle=self.angle)
        base_square = sympy.Polygon((0, 0), base_size / 2, n=4)
        border_square = sympy.Polygon(
            (0, 0), (base_size + border_size) / 2, n=4
        )

        start_point = sympy.intersection(ray, base_square)[0]
        stop_point = sympy.intersection(ray, border_square)[0]

        result = (
            start_point + (stop_point - start_point) * self.distance
        ).evalf()
        offset = (base_size + border_size) / 2

        return result[0] + offset, result[1] + offset


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
