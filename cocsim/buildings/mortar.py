from .building import BUILDINGS
from .active_building import ActiveBuilding
from .. import game
from .colliders import RectCollider


class Mortar(ActiveBuilding):
    HEALTH = [
        400.0,
        450.0,
        500.0,
        600.0,
        650.0,
        700.0,
        800.0,
        950.0,
        1100.0,
        1300.0,
        1500.0,
        1700.0,
        1950.0,
        2150.0,
        2300.0,
        2450.0,
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


BUILDINGS.append(Mortar)
