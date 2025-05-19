from .unit import Unit
from .. import game


class Barbarian(Unit):
    SPEED = 2.0

    def __init__(self, game: "game.Game"):
        super().__init__(game)
