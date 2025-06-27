from .building import BUILDINGS
from .passive_building import PassiveBuilding
from .. import game
from .colliders import RectCollider


class TownHall(PassiveBuilding):
    HEALTH = [
        450.0,
        1600.0,
        1850.0,
        2100.0,
        2400.0,
        2800.0,
        3300.0,
        3900.0,
        4600.0,
        5500.0,
        6800.0,
        7500.0,
        8200.0,
        8900.0,
        9600.0,
        10000.0,
        10400.0,
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

    def __init__(self, game: "game.Game", x: int, y: int, level: int):
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


BUILDINGS.append(TownHall)
