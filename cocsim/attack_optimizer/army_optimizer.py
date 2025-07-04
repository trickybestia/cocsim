from typing import Type

from cocsim.map_model import MapModel
from cocsim.units import Unit


class ArmyOptimizer:
    map: MapModel
    housing_space: int
    units: list[tuple[Type[Unit], int]]

    def __init__(
        self,
        map: MapModel,
        housing_space: int,
        units: list[tuple[Type[Unit], int]],
    ):
        self.map = map
        self.housing_space = housing_space
        self.units = units
