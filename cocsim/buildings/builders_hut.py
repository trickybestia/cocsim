from .passive_building import PassiveBuilding
from .. import game
from .colliders import RectCollider


class BuildersHut(PassiveBuilding):
    HEALTH = [250, 1000, 1300, 1600, 1800, 1900, 2000]
    WIDTH = 1
    HEIGHT = 1

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
