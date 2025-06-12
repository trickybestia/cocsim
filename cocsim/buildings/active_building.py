from .simple_building import SimpleBuilding
from .. import game
from .colliders import Collider
from cocsim.consts import *


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
