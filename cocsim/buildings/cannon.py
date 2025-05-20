from .active_building import ActiveBuilding
from .. import game
from .colliders import RectCollider


class Cannon(ActiveBuilding):
    HEALTH = [
        420,
        470,
        520,
        570,
        620,
        670,
        730,
        800,
        880,
        960,
        1060,
        1160,
        1260,
        1380,
        1500,
        1620,
        1740,
        1870,
        2000,
        2150,
        2250,
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
