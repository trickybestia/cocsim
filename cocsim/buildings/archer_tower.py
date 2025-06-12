from .building import BUILDINGS
from .active_building import ActiveBuilding
from .. import game
from .colliders import RectCollider


class ArcherTower(ActiveBuilding):
    HEALTH = [
        380.0,
        420.0,
        460.0,
        500.0,
        540.0,
        580.0,
        630.0,
        690.0,
        750.0,
        810.0,
        890.0,
        970.0,
        1050.0,
        1130.0,
        1230.0,
        1310.0,
        1390.0,
        1510.0,
        1600.0,
        1700.0,
        1800.0,
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


BUILDINGS.append(ArcherTower)
