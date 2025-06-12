from .. import game
from .unit import Unit


class GroundUnit(Unit):
    def __init__(self, game: "game.Game", x: float, y: float, health: float):
        super().__init__(game, x, y, health)
