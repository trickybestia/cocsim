from typing import Literal, Type

from pydantic import BaseModel

from .building import BUILDINGS
from .passive_building import PassiveBuilding
from .. import game
from .colliders import RectCollider


class DarkElixirDrillModel(BaseModel):
    name: Literal["DarkElixirDrill"]
    x: int
    y: int
    level: int


class DarkElixirDrill(PassiveBuilding):
    HEALTH = [
        800.0,
        860.0,
        920.0,
        980.0,
        1060.0,
        1160.0,
        1280.0,
        1380.0,
        1480.0,
        1550.0,
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

    @classmethod
    def model(cls) -> Type[DarkElixirDrillModel]:
        return DarkElixirDrillModel

    def __init__(self, game: "game.Game", x: int, y: int, level: int):
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

    @classmethod
    def from_model(
        cls, game: "game.Game", model: DarkElixirDrillModel
    ) -> "DarkElixirDrill":
        return cls(game, model.x, model.y, model.level)


BUILDINGS.append(DarkElixirDrill)
