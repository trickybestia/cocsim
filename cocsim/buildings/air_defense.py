from .building import BUILDINGS
from .active_building import ActiveBuilding
from .. import game
from .colliders import RectCollider


class AirDefense(ActiveBuilding):
    HEALTH = [
        800.0,
        850.0,
        900.0,
        950.0,
        1000.0,
        1050.0,
        1100.0,
        1210.0,
        1300.0,
        1400.0,
        1500.0,
        1650.0,
        1750.0,
        1850.0,
        1950.0,
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


BUILDINGS.append(AirDefense)
