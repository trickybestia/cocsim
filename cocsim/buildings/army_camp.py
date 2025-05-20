from .passive_building import PassiveBuilding
from .. import game
from .colliders import RectCollider


class ArmyCamp(PassiveBuilding):
    HEALTH = [250, 270, 290, 310, 330, 350, 400, 500, 600, 700, 800, 850, 900]
    WIDTH = 4
    HEIGHT = 4

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
