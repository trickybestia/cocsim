from typing import Literal, Type

from pydantic import BaseModel

from .. import game
from .building import BUILDINGS
from .colliders import RectCollider
from .passive_building import PassiveBuilding


class TownHallModel(BaseModel):
    name: Literal["TownHall"]
    x: int
    y: int
    level: int


class TownHall(PassiveBuilding):
    HEALTH = [
        450.0,
        1600.0,
        1850.0,
        2100.0,
        2400.0,
        2800.0,
        3300.0,
        3900.0,
        4600.0,
        5500.0,
        6800.0,
        7500.0,
        8200.0,
        8900.0,
        9600.0,
        10000.0,
        10400.0,
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
    def model(cls) -> Type[TownHallModel]:
        return TownHallModel

    def __init__(self, game: "game.Game", x: int, y: int, level: int):
        super().__init__(
            game,
            x,
            y,
            self.HEALTH[level],
            RectCollider.from_center(
                x + self.width() / 2,
                y + self.height() / 2,
                self.width() * 0.8,
                self.height() * 0.8,
            ),
        )

    @classmethod
    def from_model(cls, game: "game.Game", model: TownHallModel) -> "TownHall":
        return cls(game, model.x, model.y, model.level)


BUILDINGS.append(TownHall)
