from .passive_building import PassiveBuilding
from .. import game
from .colliders import RectCollider


class TownHall(PassiveBuilding):
    HEALTH = 450
    WIDTH = 4
    HEIGHT = 4

    def __init__(self, game: "game.Game", x: float, y: float):
        super().__init__(
            game,
            x,
            y,
            self.HEALTH,
            self.WIDTH,
            self.HEIGHT,
            RectCollider.from_center(
                x + 2, y + 2, self.WIDTH * 0.8, self.HEIGHT * 0.8
            ),
        )
