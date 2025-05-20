from .passive_building import PassiveBuilding
from .. import game
from .colliders import RectCollider


class GoldStorage(PassiveBuilding):
    HEALTH = [
        400,
        600,
        800,
        1000,
        1200,
        1400,
        1600,
        1700,
        1800,
        1900,
        2100,
        2500,
        2900,
        3300,
        3700,
        3900,
        4050,
        4200,
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
