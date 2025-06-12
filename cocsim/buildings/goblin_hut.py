from .building import BUILDINGS
from .passive_building import PassiveBuilding
from .. import game
from .colliders import RectCollider


class GoblinHut(PassiveBuilding):
    HEALTH = [250.0]

    @classmethod
    def width(cls) -> int:
        return 2

    @classmethod
    def height(cls) -> int:
        return 2

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


BUILDINGS.append(GoblinHut)
