from typing import Literal, Type

from pydantic import BaseModel

from .building import BUILDINGS
from .passive_building import PassiveBuilding
from .. import game
from .colliders import RectCollider


class ElixirCollectorModel(BaseModel):
    name: Literal["ElixirCollector"]
    x: int
    y: int
    level: int


class ElixirCollector(PassiveBuilding):
    HEALTH = [
        400.0,
        440.0,
        480.0,
        520.0,
        560.0,
        600.0,
        640.0,
        680.0,
        720.0,
        780.0,
        860.0,
        960.0,
        1080.0,
        1180.0,
        1280.0,
        1350.0,
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
    def model(cls) -> Type[ElixirCollectorModel]:
        return ElixirCollectorModel

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
        cls, game: "game.Game", model: ElixirCollectorModel
    ) -> "ElixirCollector":
        return cls(game, model.x, model.y, model.level)


BUILDINGS.append(ElixirCollector)
