from typing import Literal, Type

from pydantic import BaseModel

from .building import BUILDINGS
from .passive_building import PassiveBuilding
from .. import game
from .colliders import RectCollider


class ElixirStorageModel(BaseModel):
    name: Literal["ElixirStorage"]
    x: int
    y: int
    level: int


class ElixirStorage(PassiveBuilding):
    HEALTH = [
        400.0,
        600.0,
        800.0,
        1000.0,
        1200.0,
        1400.0,
        1600.0,
        1700.0,
        1800.0,
        1900.0,
        2100.0,
        2500.0,
        2900.0,
        3300.0,
        3700.0,
        3900.0,
        4050.0,
        4200.0,
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
    def model(cls) -> Type[ElixirStorageModel]:
        return ElixirStorageModel

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
        cls, game: "game.Game", model: ElixirStorageModel
    ) -> "ElixirStorage":
        return cls(game, model.x, model.y, model.level)


BUILDINGS.append(ElixirStorage)
