from dataclasses import dataclass


@dataclass
class Option:
    """Building configurable option, like Air or Ground targets
    for X-Bowor rotation for Air Sweeper.
    """

    name: str
    values: list[object]
