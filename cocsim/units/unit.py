from ..game import Game


class Unit:
    game: Game
    dead: bool
    x: float
    y: float

    def __init__(self, game: Game):
        self.game = game
        self.dead = False
        self.x = 0.0
        self.y = 0.0

    def tick(self, delta_t: float): ...

    def draw(self): ...
