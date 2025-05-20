from .passive_building import PassiveBuilding
from .. import game
from .colliders import RectCollider


class Laboratory(PassiveBuilding):
    HEALTH = [
        500,
        550,
        600,
        650,
        700,
        750,
        830,
        950,
        1070,
        1140,
        1210,
        1280,
        1350,
        1400,
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
