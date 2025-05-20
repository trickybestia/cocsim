from .active_building import ActiveBuilding
from .. import game
from .colliders import RectCollider


class Mortar(ActiveBuilding):
    HEALTH = [
        400,
        450,
        500,
        600,
        650,
        700,
        800,
        950,
        1100,
        1300,
        1500,
        1700,
        1950,
        2150,
        2300,
        2450,
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
