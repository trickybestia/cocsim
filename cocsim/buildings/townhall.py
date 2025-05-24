from .passive_building import PassiveBuilding
from .. import game
from .colliders import RectCollider


class TownHall(PassiveBuilding):
    HEALTH = [
        450,
        1600,
        1850,
        2100,
        2400,
        2800,
        3300,
        3900,
        4600,
        5500,
        6800,
        7500,
        8200,
        8900,
        9600,
        10000,
        10400,
    ]

    @classmethod
    def width(cls) -> int:
        return 4

    @classmethod
    def height(cls) -> int:
        return 4

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
                self.width() * 0.8,
                self.height() * 0.8,
            ),
        )
