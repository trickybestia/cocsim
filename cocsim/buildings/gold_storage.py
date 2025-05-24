from .building import BUILDINGS
from .passive_building import PassiveBuilding
from .. import game
from .colliders import RectCollider


class GoldStorage(PassiveBuilding):
    HEALTH = [
        400,
        600,
        800,
        1000,
        1200,
        1400,
        1600,
        1700,
        1800,
        1900,
        2100,
        2500,
        2900,
        3300,
        3700,
        3900,
        4050,
        4200,
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


BUILDINGS.append(GoldStorage)
