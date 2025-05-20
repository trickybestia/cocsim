from .active_building import ActiveBuilding
from .. import game
from .colliders import RectCollider


class ArcherTower(ActiveBuilding):
    HEALTH = [
        380,
        420,
        460,
        500,
        540,
        580,
        630,
        690,
        750,
        810,
        890,
        970,
        1050,
        1130,
        1230,
        1310,
        1390,
        1510,
        1600,
        1700,
        1800,
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
