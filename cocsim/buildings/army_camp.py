from typing import Literal, Type

from pydantic import BaseModel

from .. import game
from .building import BUILDINGS
from .colliders import RectCollider
from .passive_building import PassiveBuilding


class ArmyCampModel(BaseModel):
    name: Literal["ArmyCamp"]
    x: int
    y: int
    level: int


class ArmyCamp(PassiveBuilding):
    HEALTH = [
        250.0,
        270.0,
        290.0,
        310.0,
        330.0,
        350.0,
        400.0,
        500.0,
        600.0,
        700.0,
        800.0,
        850.0,
        900.0,
    ]

    @classmethod
    def width(cls) -> int:
        return 4

    @classmethod
    def height(cls) -> int:
        return 4

    @classmethod
    def levels(cls) -> int:
        return len(cls.HEALTH)

    @classmethod
    def model(cls) -> Type[ArmyCampModel]:
        return ArmyCampModel

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
    def from_model(cls, game: "game.Game", model: ArmyCampModel) -> "ArmyCamp":
        return cls(game, model.x, model.y, model.level)


BUILDINGS.append(ArmyCamp)
