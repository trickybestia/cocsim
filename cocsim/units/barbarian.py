from enum import Enum, auto

from ..buildings import Building
from .unit import Unit
from .. import game


class BarbarianState(Enum):
    MOVING = auto()
    ATTACK_COOLDOWN = auto()


class Barbarian(Unit):
    SPEED = 2.0
    RANGE = 0.4

    state: BarbarianState
    target: tuple[Building, None]

    def __init__(self, game: "game.Game"):
        super().__init__(game)

        self.state = BarbarianState.MOVING
        self.target = None

    def tick(self, delta_t):
        return super().tick(delta_t)

    def _find_path(self, building: Building): ...
