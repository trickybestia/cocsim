from typing import Union

from .. import game, buildings


class Unit:
    game: "game.Game"
    dead: bool
    x: float
    y: float

    target: Union["buildings.Building", None]

    def __init__(self, game: "game.Game", x: float, y: float):
        self.game = game
        self.dead = False
        self.x = x
        self.y = y

        self.target = None

    def tick(self, delta_t: float): ...

    def draw(self): ...
