from cocsim.consts import *

from .. import game
from .colliders import Collider
from .simple_building import SimpleBuilding


class PassiveBuilding(SimpleBuilding):
    def __init__(
        self,
        game: "game.Game",
        x: int,
        y: int,
        health: float,
        collider: Collider,
    ):
        super().__init__(game, x, y, health, collider)
