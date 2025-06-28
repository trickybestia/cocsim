from typing import Literal, Type

from pydantic import BaseModel

from .. import game
from .building import BUILDINGS
from .colliders import RectCollider
from .passive_building import PassiveBuilding


class GoblinHutModel(BaseModel):
    name: Literal["GoblinHut"]
    x: int
    y: int
    level: int


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

    @classmethod
    def model(cls) -> Type[GoblinHutModel]:
        return GoblinHutModel

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
        cls, game: "game.Game", model: GoblinHutModel
    ) -> "GoblinHut":
        return cls(game, model.x, model.y, model.level)


BUILDINGS.append(GoblinHut)
