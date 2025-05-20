from .passive_building import PassiveBuilding
from .. import game
from .colliders import RectCollider


class Barracks(PassiveBuilding):
    HEALTH = [
        250,
        290,
        330,
        370,
        420,
        470,
        520,
        580,
        650,
        730,
        810,
        900,
        980,
        1050,
        1150,
        1250,
        1350,
        1450,
    ]
    WIDTH = 3
    HEIGHT = 3

    def __init__(self, game: "game.Game", x: float, y: float, level: int):
        super().__init__(
            game,
            x,
            y,
            self.HEALTH[level],
            self.WIDTH,
            self.HEIGHT,
            RectCollider.from_center(
                x + self.WIDTH / 2,
                y + self.HEIGHT / 2,
                self.WIDTH * 0.65,
                self.HEIGHT * 0.65,
            ),
        )
