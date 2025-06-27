from .building import BUILDINGS
from .passive_building import PassiveBuilding
from .. import game
from .colliders import RectCollider


class ClanCastle(PassiveBuilding):
    HEALTH = [
        1000.0,
        1400.0,
        2000.0,
        2600.0,
        3000.0,
        3400.0,
        4000.0,
        4400.0,
        4800.0,
        5200.0,
        5400.0,
        5600.0,
        5800.0,
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

    def __init__(self, game: "game.Game", x: int, y: int, level: int):
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


BUILDINGS.append(ClanCastle)
