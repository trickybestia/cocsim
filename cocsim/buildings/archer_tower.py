from .active_building import ActiveBuilding
from .. import game
from .colliders import RectCollider


class ArcherTower(ActiveBuilding):
    HEALTH = [
        380,
        420,
        460,
        500,
        540,
        580,
        630,
        690,
        750,
        810,
        890,
        970,
        1050,
        1130,
        1230,
        1310,
        1390,
        1510,
        1600,
        1700,
        1800,
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
