from .active_building import ActiveBuilding
from .. import game
from .colliders import RectCollider


class Mortar(ActiveBuilding):
    HEALTH = [
        400,
        450,
        500,
        600,
        650,
        700,
        800,
        950,
        1100,
        1300,
        1500,
        1700,
        1950,
        2150,
        2300,
        2450,
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
