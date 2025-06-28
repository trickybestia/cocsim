from typing import Literal, Type

from pydantic import BaseModel

from .. import game
from .building import BUILDINGS
from .colliders import RectCollider
from .passive_building import PassiveBuilding


class LaboratoryModel(BaseModel):
    name: Literal["Laboratory"]
    x: int
    y: int
    level: int


class Laboratory(PassiveBuilding):
    HEALTH = [
        500.0,
        550.0,
        600.0,
        650.0,
        700.0,
        750.0,
        830.0,
        950.0,
        1070.0,
        1140.0,
        1210.0,
        1280.0,
        1350.0,
        1400.0,
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
    def model(cls) -> Type[LaboratoryModel]:
        return LaboratoryModel

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
        cls, game: "game.Game", model: LaboratoryModel
    ) -> "Laboratory":
        return cls(game, model.x, model.y, model.level)


BUILDINGS.append(Laboratory)
