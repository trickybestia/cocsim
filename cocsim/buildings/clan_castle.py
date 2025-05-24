from .passive_building import PassiveBuilding
from .. import game
from .colliders import RectCollider


class ClanCastle(PassiveBuilding):
    HEALTH = [
        1000,
        1400,
        2000,
        2600,
        3000,
        3400,
        4000,
        4400,
        4800,
        5200,
        5400,
        5600,
        5800,
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
                y + self.width() / 2,
                self.width() * 0.65,
                self.height() * 0.65,
            ),
        )
