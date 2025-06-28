from cocsim.consts import *
from cocsim.utils import distance

from .. import game
from .colliders import Collider
from .simple_building import SimpleBuilding


class ActiveBuilding(SimpleBuilding):
    def __init__(
        self,
        game: "game.Game",
        x: int,
        y: int,
        health: float,
        collider: Collider,
    ):
        super().__init__(game, x, y, health, collider)

    def splash_attack(self, x: float, y: float, radius: float, damage: float):
        for unit in self.game.units:
            if not unit.dead and distance(x, y, unit.x, unit.y) <= radius:
                unit.apply_damage(damage)
