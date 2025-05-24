from .passive_building import PassiveBuilding
from .. import game
from .colliders import RectCollider


class Laboratory(PassiveBuilding):
    HEALTH = [
        500,
        550,
        600,
        650,
        700,
        750,
        830,
        950,
        1070,
        1140,
        1210,
        1280,
        1350,
        1400,
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
