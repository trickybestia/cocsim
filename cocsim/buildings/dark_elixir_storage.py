from typing import Literal, Type

from pydantic import BaseModel

from .building import BUILDINGS
from .passive_building import PassiveBuilding
from .. import game
from .colliders import RectCollider


class DarkElixirStorageModel(BaseModel):
    name: Literal["DarkElixirStorage"]
    x: int
    y: int
    level: int


class DarkElixirStorage(PassiveBuilding):
    HEALTH = [
        2000.0,
        2200.0,
        2400.0,
        2600.0,
        2900.0,
        3200.0,
        3500.0,
        3800.0,
        4100.0,
        4300.0,
        4500.0,
        4700.0,
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
    def model(cls) -> Type[DarkElixirStorageModel]:
        return DarkElixirStorageModel

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
        cls, game: "game.Game", model: DarkElixirStorageModel
    ) -> "DarkElixirStorage":
        return cls(game, model.x, model.y, model.level)


BUILDINGS.append(DarkElixirStorage)
