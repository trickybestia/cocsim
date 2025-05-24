from .building import BUILDINGS
from .active_building import ActiveBuilding
from .. import game
from .colliders import RectCollider


class AirDefense(ActiveBuilding):
    HEALTH = [
        800,
        850,
        900,
        950,
        1000,
        1050,
        1100,
        1210,
        1300,
        1400,
        1500,
        1650,
        1750,
        1850,
        1950,
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
