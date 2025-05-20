from .passive_building import PassiveBuilding
from .. import game
from .colliders import RectCollider


class ClanCastle(PassiveBuilding):
    HEALTH = [
        1000,
        1400,
        2000,
        2600,
        3000,
        3400,
        4000,
        4400,
        4800,
        5200,
        5400,
        5600,
        5800,
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
