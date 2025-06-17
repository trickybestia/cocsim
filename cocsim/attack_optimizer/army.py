from typing import Type
from random import choice

from cocsim.units import Unit


class Army:
    units: list[tuple[Type[Unit], int]]

    @classmethod
    def randomize(
        cls, units: list[tuple[Type[Unit], int]], housing_space: int
    ) -> "Army":
        result = Army([])
        occupied_housing_space = 0

        while True:
            units = [
                t
                for t in units
                if t[0].housing_space()
                <= housing_space - occupied_housing_space
            ]

            if len(units == 0):
                break

            unit = choice(units)

            result.units.append(unit)
            occupied_housing_space += unit[0].housing_space()

        return result

    def __init__(self, units: list[tuple[Type[Unit], int]]):
        self.units = units
