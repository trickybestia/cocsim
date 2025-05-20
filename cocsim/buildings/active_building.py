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
        health: int,
        width: int,
        height: int,
        collider: Collider,
    ):
        super().__init__(game, x, y, health, width, height, collider)

        self.x = x
        self.y = y
        self.health = health
        self.width = width
        self.height = height
        self.collider = collider
