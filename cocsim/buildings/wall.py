from .building import BUILDINGS
from .passive_building import PassiveBuilding
from .. import game
from .colliders import RectCollider


class Wall(PassiveBuilding):
    HEALTH = [
        300,
        500,
        700,
        900,
        1400,
        2000,
        2500,
        3000,
        3500,
        4000,
        5000,
        7000,
        9000,
        11000,
        12500,
        13500,
        14500,
        15500,
    ]

    @classmethod
    def width(cls) -> int:
        return 1

    @classmethod
    def height(cls) -> int:
        return 1

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


BUILDINGS.append(Wall)
