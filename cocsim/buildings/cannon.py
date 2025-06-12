from .building import BUILDINGS
from .active_building import ActiveBuilding
from .. import game
from .colliders import RectCollider


class Cannon(ActiveBuilding):
    HEALTH = [
        420.0,
        470.0,
        520.0,
        570.0,
        620.0,
        670.0,
        730.0,
        800.0,
        880.0,
        960.0,
        1060.0,
        1160.0,
        1260.0,
        1380.0,
        1500.0,
        1620.0,
        1740.0,
        1870.0,
        2000.0,
        2150.0,
        2250.0,
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


BUILDINGS.append(Cannon)
