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

    @classmethod
    def width(cls) -> int:
        return 3

    @classmethod
    def height(cls) -> int:
        return 3

    @classmethod
    def levels(cls) -> int:
        return len(cls.HEALTH)

    def __init__(self, game: "game.Game", x: float, y: float, level: int):
        super().__init__(
            game,
            x,
            y,
            self.HEALTH[level],
            RectCollider.from_center(
                x + self.width() / 2,
                y + self.height() / 2,
                self.width() * 0.65,
                self.height() * 0.65,
            ),
        )
