from typing import Type

from cocsim.map import Map
from cocsim.units import Unit


class AttackOptimizer:
    map: Map
    housing_space: int
    units: list[tuple[Type[Unit], int]]

    def __init__(
        self, map: Map, housing_space: int, units: list[tuple[Type[Unit], int]]
    ):
        self.map = map
        self.housing_space = housing_space
        self.units = units
