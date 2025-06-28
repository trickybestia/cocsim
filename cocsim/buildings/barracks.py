from typing import Literal, Type

from pydantic import BaseModel

from .. import game
from .building import BUILDINGS
from .colliders import RectCollider
from .passive_building import PassiveBuilding


class BarracksModel(BaseModel):
    name: Literal["Barracks"]
    x: int
    y: int
    level: int


class Barracks(PassiveBuilding):
    HEALTH = [
        250.0,
        290.0,
        330.0,
        370.0,
        420.0,
        470.0,
        520.0,
        580.0,
        650.0,
        730.0,
        810.0,
        900.0,
        980.0,
        1050.0,
        1150.0,
        1250.0,
        1350.0,
        1450.0,
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
    def model(cls) -> Type[BarracksModel]:
        return BarracksModel

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
    def from_model(cls, game: "game.Game", model: BarracksModel) -> "Barracks":
        return cls(game, model.x, model.y, model.level)


BUILDINGS.append(Barracks)
